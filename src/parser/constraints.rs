//! Parser for ASN.1 SubType Constraints

use crate::error::Error;
use crate::structs::constraints::{
    Asn1Constraint, Elements, IntersectionSet, RangeElement, SizeElement, SubtypeElements,
    UnionSet, ValueElement,
};
use crate::tokenizer::Token;

use super::utils::{expect_keyword, expect_token, expect_token_one_of, expect_tokens};
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

    eprintln!("parse_union_set");
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
                    eprintln!("parse_intersection_set:Err");
                    if expecting_iset {
                        return Err(parse_error!("Expecting Interesection Set in a Constraint."));
                    }
                }
            }

            if !expect_token(&tokens[consumed..], Token::is_set_intersection)? {
                eprintln!("parse_intersection_set:break:IntersectionSet");
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
            eprintln!("parse_intersection_set:break:UnionSet");
            break; // Break UnionSet loop.
        } else {
            consumed += 1;
        }
    }

    Ok((UnionSet { elements }, consumed))
}

fn parse_intersection_set<'parser>(tokens: &'parser [Token]) -> Result<(Elements, usize), Error> {
    let mut consumed = 0;

    eprintln!("parse_intersection_set");
    if expect_keyword(&tokens[consumed..], "SIZE")? {
        consumed += 1;
        let (size_elements, size_elements_consumed) = parse_size_elements(&tokens[consumed..])?;
        consumed += size_elements_consumed;

        return Ok((
            Elements::Subtype(SubtypeElements::SizeConstraint(size_elements)),
            consumed,
        ));
    }

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

    Err(parse_error!("parse_intersection_set: Not Implmented"))
}

fn parse_size_elements<'parser>(tokens: &'parser [Token]) -> Result<(SizeElement, usize), Error> {
    let mut consumed = 0;

    eprintln!("parse_size_elements");
    if expect_keyword(&tokens[consumed..], "SIZE")? {
        consumed += 1;
        let values: UnionSet;
        if expect_token(&tokens[consumed..], Token::is_round_begin)? {
            consumed += 1;
            let (uset, uset_consumed) = parse_union_set(&tokens[consumed..])?;
            values = uset;
            consumed += uset_consumed;

            if !expect_token(&tokens[consumed..], Token::is_round_end)? {
                Err(unexpected_token!("')'", tokens[consumed]))
            } else {
                consumed += 1;

                Ok((SizeElement { values }, consumed))
            }
        } else {
            Err(unexpected_token!("'('", tokens[consumed]))
        }
    } else {
        Err(parse_error!("Parse Error in Size Constraint"))
    }
}

fn parse_range_elements<'parser>(tokens: &'parser [Token]) -> Result<(RangeElement, usize), Error> {
    let consumed = 0;

    fn is_min_max_keyword(token: &Token) -> bool {
        ["MIN", "MAX"]
            .iter()
            .any(|k| Token::is_given_keyword(token, k))
    }

    eprintln!("parse_range_elements");
    if expect_tokens(
        // MIN..MAX
        &tokens[consumed..],
        &[
            &[
                Token::is_value_reference,
                Token::is_numeric,
                is_min_max_keyword,
            ],
            &[Token::is_range_separator],
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
                is_min_max_keyword,
            ],
            &[Token::is_less_than],
            &[Token::is_range_separator],
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
                is_min_max_keyword,
            ],
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
        let input = "(1..3|10|11)";
        let reader = std::io::BufReader::new(std::io::Cursor::new(input));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        let constraints = parse_constraint(&tokens);
        assert!(constraints.is_ok(), "{:#?}", constraints.err());
    }
}
