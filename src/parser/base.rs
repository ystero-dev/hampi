//! Parsing for Base types

use crate::error::Error;
use crate::structs::base::{Asn1TypeInteger, NamedValue};
use crate::tokenizer::Token;

use super::utils::{expect_keyword, expect_token};

fn parse_named_maybe_value<'parser>(
    tokens: &'parser [Token],
) -> Result<((String, Option<NamedValue>), usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_value_reference)? {
        return Err(unexpected_token!("'IDENTIFIER'", tokens[consumed]));
    }
    let identifier = tokens[consumed].text.clone();
    consumed += 1;

    let named_value = match expect_token(&tokens[consumed..], Token::is_round_begin) {
        Ok(n) => {
            if n {
                consumed += 1;

                let named_value = if expect_token(&tokens[consumed..], Token::is_numeric)? {
                    let num = tokens[consumed].text.clone();
                    consumed += 1;
                    NamedValue::Number(num)
                } else if expect_token(&tokens[consumed..], Token::is_value_reference)? {
                    let valueref = tokens[consumed].text.clone();
                    consumed += 1;
                    NamedValue::ValueRef(valueref)
                } else {
                    return Err(unexpected_token!(
                        "'Reference' or 'Number'",
                        tokens[consumed]
                    ));
                };
                if !expect_token(&tokens[consumed..], Token::is_round_end)? {
                    return Err(unexpected_token!("')'", tokens[consumed]));
                }
                consumed += 1;

                Some(named_value)
            } else {
                None
            }
        }
        Err(_) => None,
    };

    Ok(((identifier, named_value), consumed))
}

fn parse_named_values<'parser>(
    tokens: &'parser [Token],
) -> Result<(Vec<(String, NamedValue)>, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{", tokens[consumed]));
    }
    consumed += 1;
    let mut values = vec![];
    loop {
        let ((identifier, named_value), named_value_consumed) =
            parse_named_maybe_value(&tokens[consumed..])?;

        if named_value.is_none() {
            return Err(parse_error!("Name(Value) expected, Value missing!"));
        }
        let named_value = named_value.unwrap();

        eprintln!(
            "identifier: {:#?}, named_value: {:#?}",
            identifier, named_value
        );
        values.push((identifier, named_value));
        consumed += named_value_consumed;

        if expect_token(&tokens[consumed..], Token::is_comma)? {
            consumed += 1;
        } else if expect_token(&tokens[consumed..], Token::is_curly_end)? {
            consumed += 1;
            break;
        } else {
            return Err(unexpected_token!(
                "'Reference' or 'Number'",
                tokens[consumed]
            ));
        }
    }

    Ok((values, consumed))
}

pub(crate) fn parse_integer_type<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1TypeInteger, usize), Error> {
    let mut consumed = 0;

    if !expect_keyword(&tokens[consumed..], "INTEGER")? {
        return Err(unexpected_token!("'INTEGER'", tokens[consumed]));
    }
    consumed += 1;

    let named_values: Option<NamedValue> = None;
    let named_values = match expect_token(&tokens[consumed..], Token::is_curly_begin) {
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

    Ok((Asn1TypeInteger { named_values }, consumed))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn parse_integer_tests() {
        struct ParseIntegerTestCase<'tc> {
            input: &'tc str,
            success: bool,
            named_values_present: bool,
            named_values_count: usize,
        }

        let test_cases = vec![
            ParseIntegerTestCase {
                input: "INTEGER",
                success: true,
                named_values_present: false,
                named_values_count: 0,
            },
            ParseIntegerTestCase {
                input: "INTEGER {a(1)}",
                success: true,
                named_values_present: true,
                named_values_count: 1,
            },
            ParseIntegerTestCase {
                input: "INTEGER {a(1), b(-10) }",
                success: true,
                named_values_present: true,
                named_values_count: 2,
            },
            ParseIntegerTestCase {
                input: "INTEGER {a(1), b(c) }",
                success: true,
                named_values_present: true,
                named_values_count: 2,
            },
            ParseIntegerTestCase {
                input: "INTEGER {a(1)}, b", // Success the training ", b" is ignored
                success: true,
                named_values_present: true,
                named_values_count: 1,
            },
            ParseIntegerTestCase {
                input: "INTEGER {a()}",
                success: false,
                named_values_present: true,
                named_values_count: 1,
            },
            ParseIntegerTestCase {
                input: "INTEGER {a(1), b}",
                success: false,
                named_values_present: true,
                named_values_count: 1,
            },
        ];

        for tc in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(tc.input));
            let tokens = tokenize(reader);
            assert!(tokens.is_ok());
            let tokens = tokens.unwrap();

            let int_type = parse_integer_type(&tokens);
            assert_eq!(int_type.is_ok(), tc.success, "{}", tc.input);

            if tc.success {
                let (int_type, _) = int_type.unwrap();
                assert_eq!(int_type.named_values.is_some(), tc.named_values_present);

                if tc.named_values_present {
                    let named_values = int_type.named_values.unwrap();
                    assert_eq!(named_values.len(), tc.named_values_count);
                }
            }
        }
    }
}
