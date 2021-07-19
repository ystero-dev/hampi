//! Handling of Sequence and Sequence Of Type

use crate::error::Error;
use crate::parser::{
    constraints::parse_constraint,
    types::parse_type,
    utils::{expect_keyword, expect_token},
    values::parse_value,
};
use crate::structs::constructed::{Asn1TypeSequence, Asn1TypeSequenceOf, SeqComponent};
use crate::structs::types::{Asn1ConstructedType, Asn1TypeKind};
use crate::tokenizer::Token;

use super::utils::parse_component;

pub(crate) fn parse_seq_or_seq_of_type<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1TypeKind, usize), Error> {
    if !expect_keyword(tokens, "SEQUENCE")? {
        return Err(unexpected_token!("'SEQUENCE'", tokens[0]));
    }

    if expect_token(&tokens[1..], Token::is_curly_begin)? {
        parse_sequence_type(tokens)
    } else {
        parse_sequence_of_type(tokens)
    }
}

fn parse_sequence_type<'parser>(tokens: &'parser [Token]) -> Result<(Asn1TypeKind, usize), Error> {
    let mut consumed = 0;
    // Initial 'SEQUENCE' is consumed by the caller. We start with '{'

    consumed += 1; // For the SEQUENCE

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{'", tokens[consumed]));
    }
    consumed += 1;

    let mut root_components = vec![];
    let additions = vec![];
    loop {
        let (component, component_consumed) = match parse_component(&tokens[consumed..]) {
            Ok(result) => (Some(result.0), result.1),
            Err(_) => (None, 0),
        };
        consumed += component_consumed;

        if let Some(component) = component {
            let optional = if expect_keyword(&tokens[consumed..], "OPTIONAL")? {
                consumed += 1;
                true
            } else {
                false
            };

            let default = if expect_keyword(&tokens[consumed..], "DEFAULT")? {
                consumed += 1;
                let (value, value_consumed) = parse_value(&tokens[consumed..])?;
                consumed += value_consumed;
                Some(value)
            } else {
                None
            };

            if default.is_some() && optional {
                return Err(parse_error!(
                    "Both OPTIONAL and DEFAULT not allowed for a value!"
                ));
            }

            root_components.push(SeqComponent {
                component,
                optional,
                default,
            });
        }

        if expect_token(&tokens[consumed..], Token::is_comma)? {
            consumed += 1;
        }

        if expect_token(&tokens[consumed..], Token::is_extension)? {
            consumed += 1;
            if expect_token(&tokens[consumed..], Token::is_comma)? {
                consumed += 1;
            }
        }

        if expect_token(&tokens[consumed..], Token::is_curly_end)? {
            consumed += 1;
            break;
        }
    }

    Ok((
        Asn1TypeKind::Constructed(Asn1ConstructedType::Sequence(Asn1TypeSequence {
            root_components,
            additions,
        })),
        consumed,
    ))
}

fn parse_sequence_of_type<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1TypeKind, usize), Error> {
    let mut consumed = 0;

    // Initial SEQUENCE is already consumed.
    consumed += 1;

    let (size, size_consumed) = match parse_constraint(&tokens[consumed..]) {
        Ok(result) => (Some(result.0), result.1),
        Err(_) => (None, 0),
    };
    consumed += size_consumed;

    if !expect_keyword(&tokens[consumed..], "OF")? {
        return Err(unexpected_token!("'OF'", tokens[consumed]));
    }
    consumed += 1;

    let (ty, ty_consumed) = parse_type(&tokens[consumed..])?;
    consumed += ty_consumed;

    let ty = Box::new(ty);
    Ok((
        Asn1TypeKind::Constructed(Asn1ConstructedType::SequenceOf(Asn1TypeSequenceOf {
            size,
            ty,
        })),
        consumed,
    ))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn parse_sequence_test_cases() {
        struct ParseSequenceTestCase<'tc> {
            input: &'tc str,
            success: bool,
            root_components_count: usize,
            additional_components_count: usize,
            consumed_tokens: usize,
        }

        let test_cases = vec![
            ParseSequenceTestCase {
                input: " SEQUENCE {} ",
                success: true,
                root_components_count: 0,
                additional_components_count: 0,
                consumed_tokens: 3,
            },
            ParseSequenceTestCase {
                input: " SEQUENCE { a INTEGER, b BOOLEAN } ",
                success: true,
                root_components_count: 2,
                additional_components_count: 0,
                consumed_tokens: 8,
            },
            ParseSequenceTestCase {
                input: " SEQUENCE { a INTEGER, b BOOLEAN OPTIONAL} ",
                success: true,
                root_components_count: 2,
                additional_components_count: 0,
                consumed_tokens: 9,
            },
            ParseSequenceTestCase {
                input: " SEQUENCE { a INTEGER, b BOOLEAN OPTIONAL, ..., c CHOICE { d INTEGER, e Enum}} ",
                success: true,
                root_components_count: 3,
                additional_components_count: 0,
                consumed_tokens: 21,
            },
            ParseSequenceTestCase {
                input: " SEQUENCE (SIZE(1..maxnoofeNBX2TLAs)) OF TransportLayerAddress",
                success: true,
                root_components_count: 3,
                additional_components_count: 0,
                consumed_tokens: 21,
            },
        ];

        for tc in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(tc.input));
            let tokens = tokenize(reader);
            assert!(tokens.is_ok());
            let tokens = tokens.unwrap();

            let sequence = parse_seq_or_seq_of_type(&tokens);
            assert_eq!(
                sequence.is_ok(),
                tc.success,
                "{}:{}",
                tc.input,
                if tc.success {
                    format!("{:#?}", sequence.err())
                } else {
                    format!("{:#?}", sequence.ok())
                }
            );

            if tc.success {
                let (seq, seq_consumed) = sequence.unwrap();
                if let Asn1TypeKind::Constructed(Asn1ConstructedType::Sequence(seq)) = seq {
                    assert_eq!(seq_consumed, tc.consumed_tokens, "{}", tc.input);
                    assert_eq!(
                        seq.root_components.len(),
                        tc.root_components_count,
                        "{}",
                        tc.input
                    );
                    assert_eq!(
                        seq.additions.len(),
                        tc.additional_components_count,
                        "{}",
                        tc.input
                    );
                }
            }
        }
    }
}
