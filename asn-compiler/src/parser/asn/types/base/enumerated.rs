//! Functionality related to parsing 'ENUMERATED' ASN.1 Type

use crate::tokenizer::Token;
use anyhow::Result;

use crate::parser::utils::{expect_keyword, expect_one_of_tokens, expect_token};

use crate::parser::asn::structs::types::base::{Asn1TypeEnumerated, EnumValue};

use super::utils::parse_named_maybe_value;

// Parses values in an Enum. Used for parsing values either in the root or extension.
fn parse_enum_values(tokens: &[Token]) -> Result<(Vec<EnumValue>, usize)> {
    let mut consumed = 0;

    let mut values = vec![];
    loop {
        let (named_value, named_value_consumed) = parse_named_maybe_value(&tokens[consumed..])?;
        let value = EnumValue {
            name: named_value.0,
            value: named_value.1,
        };

        values.push(value);
        consumed += named_value_consumed;

        if expect_token(&tokens[consumed..], Token::is_comma)? {
            consumed += 1;
        }

        if expect_one_of_tokens(
            &tokens[consumed..],
            &[Token::is_extension, Token::is_curly_end],
        )? {
            // Don't consume, let caller consume it.
            break;
        }
    }
    Ok((values, consumed))
}

// Parse an enumerated type
pub(crate) fn parse_enumerated_type(tokens: &[Token]) -> Result<(Asn1TypeEnumerated, usize)> {
    let mut consumed = 0;

    if !expect_keyword(tokens, "ENUMERATED")? {
        return Err(unexpected_token!("ENUMERATED", tokens[0]).into());
    }
    consumed += 1;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{'", tokens[consumed]).into());
    }
    consumed += 1;

    let (root_values, root_values_consumed) = parse_enum_values(&tokens[consumed..])?;
    consumed += root_values_consumed;

    let ext_marker_index = if expect_token(&tokens[consumed..], Token::is_extension)? {
        consumed += 1;
        Some(root_values.len() + 1)
    } else {
        None
    };

    // one(1), ...,
    if expect_token(&tokens[consumed..], Token::is_comma)? {
        consumed += 1;
    }

    let (ext_values, ext_values_consumed) = match parse_enum_values(&tokens[consumed..]) {
        Ok(result) => result,
        Err(_) => (vec![], 0),
    };
    consumed += ext_values_consumed;

    if !expect_token(&tokens[consumed..], Token::is_curly_end)? {
        return Err(unexpected_token!("'}'", tokens[consumed]).into());
    }
    consumed += 1;

    Ok((
        Asn1TypeEnumerated {
            root_values,
            ext_marker_index,
            ext_values,
        },
        consumed,
    ))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn parse_enumerated_type_tests() {
        struct ParseEnumeratedTestCase<'tc> {
            input: &'tc str,
            success: bool,
            root_values_count: usize,
            ext_marker_present: bool,
            ext_marker_idx: usize,
            ext_values_count: usize,
            tokens_consumed: usize,
        }

        let test_cases = vec![
            ParseEnumeratedTestCase {
                input: " ENUMERATED { one(1), two(2) }",
                success: true,
                root_values_count: 2,
                ext_marker_present: false,
                ext_marker_idx: 0,
                ext_values_count: 0,
                tokens_consumed: 12,
            },
            ParseEnumeratedTestCase {
                input: " ENUMERATED { one(1), two(2), ... }",
                success: true,
                root_values_count: 2,
                ext_marker_present: true,
                ext_marker_idx: 3,
                ext_values_count: 0,
                tokens_consumed: 14,
            },
            ParseEnumeratedTestCase {
                input: " ENUMERATED { one(1), two(2), ..., three, four(4) }",
                success: true,
                root_values_count: 2,
                ext_marker_present: true,
                ext_marker_idx: 3,
                ext_values_count: 2,
                tokens_consumed: 21,
            },
            ParseEnumeratedTestCase {
                input: " ENUMERATED { one(1), two(2), ... }, ...", // Success
                success: true,
                root_values_count: 2,
                ext_marker_present: true,
                ext_marker_idx: 3,
                ext_values_count: 0,
                tokens_consumed: 14,
            },
            ParseEnumeratedTestCase {
                input: " ENUMERATED { one(1), two(2), ..., three, four(4), ... }",
                success: false,
                root_values_count: 0,
                ext_marker_present: false,
                ext_marker_idx: 0,
                ext_values_count: 0,
                tokens_consumed: 0,
            },
            ParseEnumeratedTestCase {
                input: " ENUMERATED { one(1), two(2), ..., three, four(4), ... ",
                success: false,
                root_values_count: 0,
                ext_marker_present: false,
                ext_marker_idx: 0,
                ext_values_count: 0,
                tokens_consumed: 0,
            },
        ];

        for tc in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(tc.input));
            let tokens = tokenize(reader);
            assert!(tokens.is_ok());
            let tokens = tokens.unwrap();

            let enum_type = parse_enumerated_type(&tokens);
            assert_eq!(enum_type.is_ok(), tc.success, "{}", tc.input);

            if tc.success {
                let (enum_type, enum_type_consumed) = enum_type.unwrap();
                assert_eq!(tc.tokens_consumed, enum_type_consumed, "{}", tc.input);

                assert_eq!(
                    enum_type.root_values.len(),
                    tc.root_values_count,
                    "{}",
                    tc.input
                );

                assert_eq!(
                    enum_type.ext_marker_index.is_some(),
                    tc.ext_marker_present,
                    "{}",
                    tc.input
                );
                if tc.ext_marker_present {
                    assert_eq!(
                        enum_type.ext_marker_index.unwrap(),
                        tc.ext_marker_idx,
                        "{}",
                        tc.input
                    );
                }
                assert_eq!(
                    enum_type.ext_values.len(),
                    tc.ext_values_count,
                    "{}",
                    tc.input
                );
            }
        }
    }
}
