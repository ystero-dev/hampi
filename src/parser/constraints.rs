//! Parser for ASN.1 SubType Constraints

use crate::error::Error;
use crate::structs::constraints::*;
use crate::tokenizer::Token;

use super::types::parse_type;
use super::utils::{
    expect_keyword, expect_one_of_keywords, expect_one_of_tokens, expect_token, expect_tokens,
};
use super::values::parse_value;

pub(super) fn parse_constraints<'parser>(
    tokens: &'parser [Token],
) -> Result<(Vec<Asn1Constraint>, usize), Error> {
    let mut consumed = 0;

    let mut constraints = vec![];
    loop {
        match parse_constraint(&tokens[consumed..]) {
            Ok(result) => {
                constraints.push(result.0);
                consumed += result.1;
            }
            Err(_) => {
                break;
            }
        }
    }
    Ok((constraints, consumed))
}

pub(crate) fn parse_constraint<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Constraint, usize), Error> {
    if let Ok(subtype) = parse_subtype_constraint(tokens) {
        Ok(subtype)
    } else if let Ok(table) = parse_table_constraint(tokens) {
        Ok(table)
    } else if let Ok(containing) = parse_contents_constraint(tokens) {
        Ok(containing)
    } else {
        Err(parse_error!(
            "Parsing of this constraint not yet supported!"
        ))
    }
}

fn parse_table_constraint<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Constraint, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_round_begin)? {
        return Err(unexpected_token!("'('", tokens[0]));
    }
    consumed += 1;

    // First Simple Table - Must succeed
    let table = if expect_tokens(
        &tokens[consumed..],
        &[
            &[Token::is_curly_begin],
            &[Token::is_object_set_reference],
            &[Token::is_curly_end],
        ],
    )? {
        tokens[consumed + 1].text.clone()
    } else {
        return Err(parse_error!("Failed to parse Simple Table Constraint."));
    };
    consumed += 3;

    let component = match expect_tokens(
        &tokens[consumed..],
        &[
            &[Token::is_curly_begin],
            &[Token::is_at_component_list],
            &[Token::is_curly_end],
        ],
    ) {
        Ok(result) => {
            if result {
                let c = tokens[consumed + 1].text.clone();
                consumed += 3;
                Some(c)
            } else {
                None
            }
        }
        Err(_) => None,
    };

    let constraint = if component.is_some() {
        Asn1Constraint::Table(TableConstraint::CompRel(ComponentRelation {
            table,
            component: component.unwrap(),
        }))
    } else {
        Asn1Constraint::Table(TableConstraint::Simple(ObjectSet::DefinedObjectSet(table)))
    };

    if !expect_token(&tokens[consumed..], Token::is_round_end)? {
        return Err(unexpected_token!("')'", tokens[consumed]));
    }
    consumed += 1;

    Ok((constraint, consumed))
}

fn parse_subtype_constraint<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Constraint, usize), Error> {
    let (element_set, element_set_consumed) = parse_element_set(tokens)?;
    Ok((Asn1Constraint::Subtype(element_set), element_set_consumed))
}

fn parse_element_set<'parser>(tokens: &'parser [Token]) -> Result<(ElementSet, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_round_begin)? {
        return Err(unexpected_token!("'('", tokens[0]));
    }
    consumed += 1;

    let (root_elements, root_consumed) = parse_union_set(&tokens[consumed..])?;
    consumed += root_consumed;

    if root_elements.elements.is_empty() {
        return Err(parse_error!("Empty Set in a Constraint!"));
    }

    let mut additional_elements = None;
    if expect_token(&tokens[consumed..], Token::is_comma)? {
        consumed += 1;

        // Extension Marker
        if !expect_token(&tokens[consumed..], Token::is_extension)? {
            return Err(unexpected_token!("'...'", tokens[consumed]));
        }
        consumed += 1;

        // Potentially Empty additional_elements
        match parse_union_set(&tokens[consumed..]) {
            Ok(result) => {
                additional_elements = Some(result.0);
                consumed += result.1;
            }
            Err(_) => {}
        }
    }

    if !expect_token(&tokens[consumed..], Token::is_round_end)? {
        return Err(unexpected_token!("')'", tokens[consumed]));
    }
    consumed += 1;

    Ok((
        ElementSet {
            root_elements,
            additional_elements,
        },
        consumed,
    ))
}

fn parse_union_set<'parser>(tokens: &'parser [Token]) -> Result<(UnionSet, usize), Error> {
    let mut consumed = 0;

    let mut elements = vec![];
    // UnionSet Loop
    loop {
        // IntersectionSet Loop
        let mut iset_elements = vec![];
        let mut expecting_iset = false;
        loop {
            match parse_intersection_set(&tokens[consumed..]) {
                Ok(result) => {
                    iset_elements.push(result.0);
                    consumed += result.1;
                }
                Err(_) => {
                    if expecting_iset {
                        return Err(parse_error!("Expecting Interesection Set in a Constraint."));
                    }
                }
            }

            if !expect_token(&tokens[consumed..], Token::is_set_intersection)? {
                break; // Break intersection Set loop.
            } else {
                expecting_iset = true;
                consumed += 1;
            }
        }
        if !iset_elements.is_empty() {
            elements.push(IntersectionSet {
                elements: iset_elements,
            });
        }

        if !expect_token(&tokens[consumed..], Token::is_set_union)? {
            break; // Break UnionSet loop.
        } else {
            consumed += 1;
        }
    }

    Ok((UnionSet { elements }, consumed))
}

// Just like parse_union_set, except it consumes the wrapping `(` and `)`
//
// This avoid having to write a lot of boiler-plate code to check for `(` or `)` in a few
// functions (typically inside `parse_intersection_set`.)
fn parse_intersection_set<'parser>(tokens: &'parser [Token]) -> Result<(Elements, usize), Error> {
    let mut consumed = 0;

    // First try to Parse a Size
    if expect_one_of_keywords(&tokens[consumed..], &["SIZE", "FROM"])? {
        // If we come inside, following is guaranteed. to succeed.
        let variant = if expect_keyword(&tokens[consumed..], "SIZE").unwrap() {
            SubtypeElements::SizeConstraint
        } else {
            SubtypeElements::PermittedAlphabet
        };
        consumed += 1;

        if expect_token(&tokens[consumed..], Token::is_round_begin)? {
            let (element_set, element_set_consumed) = parse_element_set(&tokens[consumed..])?;
            consumed += element_set_consumed;

            return Ok((Elements::Subtype(variant(element_set)), consumed));
        }
    }

    // Parse Range Value
    match parse_range_elements(&tokens[consumed..]) {
        Ok(result) => {
            let range_elements = result.0;
            consumed += result.1;

            return Ok((
                Elements::Subtype(SubtypeElements::ValueRange(range_elements)),
                consumed,
            ));
        }
        Err(_) => {}
    }

    // Parse nested UnionSet Constraint
    if expect_token(&tokens[consumed..], Token::is_round_begin)? {
        let (element_set, element_set_consumed) = parse_element_set(&tokens[consumed..])?;
        consumed += element_set_consumed;

        return Ok((Elements::Set(element_set), consumed));
    }

    // Parse a simple `Value`
    match parse_value(&tokens[consumed..]) {
        Ok(result) => {
            let value = result.0;
            consumed += result.1;

            return Ok((
                Elements::Subtype(SubtypeElements::SingleValue(ValueElement { value })),
                consumed,
            ));
        }
        Err(_) => {}
    }

    // Parse ContainedSubtype. Note: While the actual grammar specifies `Type` production, In
    // reality, this should just be a TypeReference. But we parse it as a full Type regardless.
    match parse_type(&tokens[consumed..]) {
        Ok(result) => {
            let parsed_type = result.0;
            consumed += result.1;

            return Ok((
                Elements::Subtype(SubtypeElements::ConstrainedSubtype(parsed_type)),
                consumed,
            ));
        }
        Err(_) => {}
    }

    Err(parse_error!("parse_intersection_set: Not Implmented"))
}

// Parses a Range Value, supports all possible formats.
//
// If parsing fails (tokens of not adequate length or tokens don't match) returns an Error. The
// caller should do the error handling. Note: Typically caller will simply say Oh it didn't match,
// let's try next.
fn parse_range_elements<'parser>(tokens: &'parser [Token]) -> Result<(RangeElement, usize), Error> {
    let mut consumed = 0;

    fn is_min_max_keyword(token: &Token) -> bool {
        ["MIN", "MAX"]
            .iter()
            .any(|k| Token::is_given_keyword(token, k))
    }

    let (lower, lower_consumed) = match parse_value(&tokens[consumed..]) {
        Ok(result) => (result.0, result.1),
        Err(_) => {
            if expect_token(&tokens[consumed..], is_min_max_keyword)? {
                (tokens[consumed].text.clone(), 1)
            } else {
                return Err(unexpected_token!(
                    "'MIN', 'MAX' or 'Value'",
                    tokens[consumed]
                ));
            }
        }
    };
    consumed += lower_consumed;

    if !expect_one_of_tokens(
        &tokens[consumed..],
        &[Token::is_less_than, Token::is_range_separator],
    )? {
        return Err(unexpected_token!("'<' or '..'", tokens[consumed]));
    }

    let lower_inclusive = if expect_token(&tokens[consumed..], Token::is_less_than)? {
        consumed += 1;
        false
    } else {
        true
    };
    consumed += 1; // The '..' Token

    let upper_inclusive = if expect_token(&tokens[consumed..], Token::is_less_than)? {
        consumed += 1;
        false
    } else {
        true
    };

    let (upper, upper_consumed) = match parse_value(&tokens[consumed..]) {
        Ok(result) => (result.0, result.1),
        Err(_) => {
            if expect_token(&tokens[consumed..], is_min_max_keyword)? {
                (tokens[consumed].text.clone(), 1)
            } else {
                return Err(unexpected_token!(
                    "'MIN', 'MAX' or 'Value'",
                    tokens[consumed]
                ));
            }
        }
    };
    consumed += upper_consumed;

    Ok((
        RangeElement {
            lower,
            lower_inclusive,
            upper,
            upper_inclusive,
        },
        consumed,
    ))
}

fn parse_contents_constraint<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Constraint, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_round_begin)? {
        return Err(unexpected_token!("'('", tokens[consumed]));
    }
    consumed += 1;

    if !expect_keyword(&tokens[consumed..], "CONTAINING")? {
        return Err(unexpected_token!("'CONTAINING'", tokens[consumed]));
    }
    consumed += 1;

    let containing = if expect_token(&tokens[consumed..], Token::is_type_reference)? {
        tokens[consumed].text.clone()
    } else {
        eprintln!("11115");
        return Err(unexpected_token!("'TYPE Reference'", tokens[consumed]));
    };
    consumed += 1;

    if !expect_token(&tokens[consumed..], Token::is_round_end)? {
        return Err(unexpected_token!("')'", tokens[consumed]));
    }
    consumed += 1;

    let encodedby = None;
    eprintln!("11114");
    Ok((
        Asn1Constraint::Contents(ContentsConstraint {
            containing,
            encodedby,
        }),
        consumed,
    ))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn parse_subtype_constraint_testcase() {
        struct ParseConstraintTestCase<'tc> {
            input: &'tc str,                   // Input String
            success: bool,                     // Check whether the constraint result is `is_ok`
            root_elements_count: usize,        // Members in root_elements
            additional_elements_present: bool, // Are additional Elements present?
            additional_elements_count: usize,  // Members in additional elements
        }

        let test_cases = vec![
            ParseConstraintTestCase {
                input: "(SIZE(1..10))",
                success: true,
                root_elements_count: 1,
                additional_elements_present: false,
                additional_elements_count: 0,
            },
            ParseConstraintTestCase {
                input: "(SIZE(1..150, ...))",
                success: true,
                root_elements_count: 1,
                additional_elements_present: false,
                additional_elements_count: 0,
            },
            ParseConstraintTestCase {
                input: r#"(FROM("a".."z"))"#,
                success: true,
                root_elements_count: 1,
                additional_elements_present: false,
                additional_elements_count: 0,
            },
            ParseConstraintTestCase {
                input: r#"(FROM ({0, 0, 3, 112}..{0, 0, 3, 207}))"#,
                success: true,
                root_elements_count: 1,
                additional_elements_present: false,
                additional_elements_count: 0,
            },
            // FIXME: Add more test cases for subtype constraints
        ];
        for tc in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(tc.input));
            let tokens = tokenize(reader);
            assert!(tokens.is_ok());
            let tokens = tokens.unwrap();

            let constraint = parse_subtype_constraint(&tokens);
            assert_eq!(
                constraint.is_ok(),
                tc.success,
                "{:#?}, {:#?}",
                tc.input,
                constraint.err()
            );

            if tc.success {
                let (elem_set, elem_set_consumed) = constraint.unwrap();

                if let Asn1Constraint::Subtype(constraint) = elem_set {
                    assert_eq!(elem_set_consumed, tokens.len(), "{:#?}", constraint);
                    assert_eq!(
                        constraint.root_elements.elements.len(),
                        tc.root_elements_count,
                        "{:#?}",
                        constraint
                    );

                    assert_eq!(
                        constraint.additional_elements.is_some(),
                        tc.additional_elements_present,
                        "{:#?}",
                        constraint
                    );

                    if constraint.additional_elements.is_some() {
                        assert_eq!(
                            &constraint
                                .additional_elements
                                .as_ref()
                                .unwrap()
                                .elements
                                .len(),
                            &tc.additional_elements_count,
                            "{:#?}",
                            constraint
                        );
                    }
                } else {
                    assert!(false, "Expected Subtype Constraint, Found {:#?}", elem_set);
                }
            }
        }
    }

    #[test]
    fn parse_table_constraint_testcases() {
        // FIXME: Add test cases
        assert!(true);
    }
}
