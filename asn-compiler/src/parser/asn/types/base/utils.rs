//! Utility functions used by base types

use crate::tokenizer::Token;
use anyhow::Result;

use crate::parser::utils::expect_token;

use crate::parser::asn::structs::types::base::NamedValue;

// Parse a name(value). `(value)` component is optional
pub(crate) fn parse_named_maybe_value(
    tokens: &[Token],
) -> Result<((String, Option<NamedValue>), usize)> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_value_reference)? {
        return Err(unexpected_token!("'IDENTIFIER'", tokens[consumed]).into());
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
                    return Err(
                        unexpected_token!("'Reference' or 'Number'", tokens[consumed]).into(),
                    );
                };
                if !expect_token(&tokens[consumed..], Token::is_round_end)? {
                    return Err(unexpected_token!("')'", tokens[consumed]).into());
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

pub(crate) fn parse_named_values(tokens: &[Token]) -> Result<(Vec<(String, NamedValue)>, usize)> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{", tokens[consumed]).into());
    }
    consumed += 1;
    let mut values = vec![];
    loop {
        let ((identifier, named_value), named_value_consumed) =
            parse_named_maybe_value(&tokens[consumed..])?;

        if named_value.is_none() {
            return Err(parse_error!("Name(Value) expected, Value missing!").into());
        }
        let named_value = named_value.unwrap();

        values.push((identifier, named_value));
        consumed += named_value_consumed;

        if expect_token(&tokens[consumed..], Token::is_comma)? {
            consumed += 1;
        } else if expect_token(&tokens[consumed..], Token::is_curly_end)? {
            consumed += 1;
            break;
        } else {
            return Err(unexpected_token!("'Reference' or 'Number'", tokens[consumed]).into());
        }
    }

    Ok((values, consumed))
}
