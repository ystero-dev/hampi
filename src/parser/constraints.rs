//! Parser for ASN.1 SubType Constraints

use crate::error::Error;
use crate::structs::constraints::{
    Asn1Constraint, Elements, IntersectionSet, RangeElement, SubtypeElements, UnionSet,
    UnionSetElement, ValueElement,
};
use crate::tokenizer::Token;

use super::types::parse_type;
use super::utils::{expect_keyword, expect_one_of_keywords, expect_token, expect_tokens};
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

fn parse_constraint<'parser>(tokens: &'parser [Token]) -> Result<(Asn1Constraint, usize), Error> {
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

        // Potentially Empty additional_elements
        consumed += 1;
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
        Asn1Constraint {
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
fn parse_enclosed_union_set<'parser>(tokens: &'parser [Token]) -> Result<(UnionSet, usize), Error> {
    let mut consumed = 0;
    if expect_token(&tokens[consumed..], Token::is_round_begin)? {
        consumed += 1;

        let (union_set, union_set_consumed) = parse_union_set(&tokens[consumed..])?;
        consumed += union_set_consumed;

        if !expect_token(&tokens[consumed..], Token::is_round_end)? {
            return Err(unexpected_token!("')'", tokens[consumed]));
        } else {
            consumed += 1;
            return Ok((union_set, consumed));
        }
    } else {
        return Err(unexpected_token!("'('", tokens[consumed]));
    }
}

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
            let (values, values_consumed) = parse_enclosed_union_set(&tokens[consumed..])?;
            consumed += values_consumed;

            return Ok((
                Elements::Subtype(variant(UnionSetElement { values })),
                consumed,
            ));
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
        let (union_set, union_set_consumed) = parse_enclosed_union_set(&tokens[consumed..])?;
        consumed += union_set_consumed;

        return Ok((Elements::ElementSet(union_set), consumed));
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
    let consumed = 0;

    fn is_min_max_keyword(token: &Token) -> bool {
        ["MIN", "MAX"]
            .iter()
            .any(|k| Token::is_given_keyword(token, k))
    }

    if expect_tokens(
        // MIN..MAX
        &tokens[consumed..],
        &[
            &[
                Token::is_value_reference,
                Token::is_numeric,
                Token::is_tstring,
                is_min_max_keyword,
            ],
            &[Token::is_range_separator],
            &[
                Token::is_value_reference,
                Token::is_numeric,
                Token::is_tstring,
                is_min_max_keyword,
            ],
        ],
    )? {
        Ok((
            RangeElement {
                lower: tokens[0].text.clone(),
                lower_inclusive: true,
                upper: tokens[2].text.clone(),
                upper_inclusive: true,
            },
            3,
        ))
    } else if expect_tokens(
        // MIN<..MAX
        &tokens[consumed..],
        &[
            &[
                Token::is_value_reference,
                Token::is_numeric,
                Token::is_tstring,
                is_min_max_keyword,
            ],
            &[Token::is_less_than],
            &[Token::is_range_separator],
            &[
                Token::is_value_reference,
                Token::is_numeric,
                Token::is_tstring,
                is_min_max_keyword,
            ],
        ],
    )? {
        Ok((
            RangeElement {
                lower: tokens[0].text.clone(),
                lower_inclusive: false,
                upper: tokens[3].text.clone(),
                upper_inclusive: true,
            },
            4,
        ))
    } else if expect_tokens(
        // MIN..<MAX
        &tokens[consumed..],
        &[
            &[
                Token::is_value_reference,
                Token::is_numeric,
                Token::is_tstring,
                is_min_max_keyword,
            ],
            &[Token::is_range_separator],
            &[Token::is_less_than],
            &[
                Token::is_value_reference,
                Token::is_numeric,
                Token::is_tstring,
                is_min_max_keyword,
            ],
        ],
    )? {
        Ok((
            RangeElement {
                lower: tokens[0].text.clone(),
                lower_inclusive: true,
                upper: tokens[3].text.clone(),
                upper_inclusive: false,
            },
            4,
        ))
    } else if expect_tokens(
        // MIN<..<MAX
        &tokens[consumed..],
        &[
            &[
                Token::is_value_reference,
                Token::is_numeric,
                is_min_max_keyword,
            ],
            &[Token::is_less_than],
            &[Token::is_range_separator],
            &[Token::is_less_than],
            &[
                Token::is_value_reference,
                Token::is_numeric,
                is_min_max_keyword,
            ],
        ],
    )? {
        Ok((
            RangeElement {
                lower: tokens[0].text.clone(),
                lower_inclusive: false,
                upper: tokens[4].text.clone(),
                upper_inclusive: false,
            },
            5,
        ))
    } else {
        Err(parse_error!("Don't care!"))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn parse_constraint_testcase() {
        let input = r#"("a".."b" | "c".."d" |foo|10|11|SIZE(0..100) ^ FROM("a".."f")|(1|2),...)"#;
        let reader = std::io::BufReader::new(std::io::Cursor::new(input));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        let constraints = parse_constraint(&tokens);
        assert!(constraints.is_ok(), "{:#?}", constraints.err());
    }
}
