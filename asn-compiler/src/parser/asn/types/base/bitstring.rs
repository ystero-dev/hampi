//! Parser for bitstring type.

use crate::error::Error;
use crate::tokenizer::Token;

use crate::parser::utils::{expect_keywords, expect_token};

use crate::parser::asn::structs::types::base::Asn1TypeBitString;

use super::utils::parse_named_values;

pub(crate) fn parse_bitstring_type(tokens: &[Token]) -> Result<(Asn1TypeBitString, usize), Error> {
    let mut consumed = 0;

    if !expect_keywords(&tokens[consumed..], &["BIT", "STRING"])? {
        return Err(unexpected_token!("'BIT STRING'", tokens[consumed]));
    }
    consumed += 2;

    let named_bits = match expect_token(&tokens[consumed..], Token::is_curly_begin) {
        Ok(c) => {
            if c {
                let (numbers_list, numbers_list_consumed) =
                    parse_named_values(&tokens[consumed..])?;
                consumed += numbers_list_consumed;
                Some(numbers_list)
            } else {
                None
            }
        }
        Err(_) => None,
    };

    Ok((Asn1TypeBitString { named_bits }, consumed))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn parse_bitstring_type_tests() {
        struct ParseBitStringTestCase<'tc> {
            input: &'tc str,
            success: bool,
            named_bits_present: bool,
            named_bits_count: usize,
            tokens_consumed: usize,
        }

        let test_cases = vec![
            ParseBitStringTestCase {
                input: "BIT STRING",
                success: true,
                named_bits_present: false,
                named_bits_count: 0,
                tokens_consumed: 2,
            },
            ParseBitStringTestCase {
                input: "BIT STRING {a(1)}",
                success: true,
                named_bits_present: true,
                named_bits_count: 1,
                tokens_consumed: 8,
            },
            ParseBitStringTestCase {
                input: "BIT STRING {a(1), b(c) }",
                success: true,
                named_bits_present: true,
                named_bits_count: 2,
                tokens_consumed: 13,
            },
            ParseBitStringTestCase {
                input: "BIT STRING {a(1)}, b", // Success the training ", b" is ignored
                success: true,
                named_bits_present: true,
                named_bits_count: 1,
                tokens_consumed: 8,
            },
            ParseBitStringTestCase {
                input: "BIT STRING {a()}",
                success: false,
                named_bits_present: false,
                named_bits_count: 0,
                tokens_consumed: 0,
            },
            ParseBitStringTestCase {
                input: "BIT STRING {a(1), b}",
                success: false,
                named_bits_present: false,
                named_bits_count: 0,
                tokens_consumed: 0,
            },
            ParseBitStringTestCase {
                input: "BIT STRING {a(1)",
                success: false,
                named_bits_present: false,
                named_bits_count: 0,
                tokens_consumed: 0,
            },
        ];

        for tc in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(tc.input));
            let tokens = tokenize(reader);
            assert!(tokens.is_ok());
            let tokens = tokens.unwrap();

            let bitstr_type = parse_bitstring_type(&tokens);
            assert_eq!(bitstr_type.is_ok(), tc.success, "{}", tc.input);

            if tc.success {
                let (bitstr_type, bitstr_type_consumed) = bitstr_type.unwrap();
                assert_eq!(bitstr_type_consumed, tc.tokens_consumed, "{}", tc.input);
                assert_eq!(
                    bitstr_type.named_bits.is_some(),
                    tc.named_bits_present,
                    "{}",
                    tc.input
                );

                if tc.named_bits_present {
                    let named_bits = bitstr_type.named_bits.unwrap();
                    assert_eq!(named_bits.len(), tc.named_bits_count, "{}", tc.input);
                }
            }
        }
    }
}
