//! Utility functions used by base types

use crate::error::Error;
use crate::structs::base::NamedValue;
use crate::tokenizer::Token;

use crate::parser::utils::expect_token;
/// Parse a name(value). `(value)` component is optional
pub(crate) fn parse_named_maybe_value<'parser>(
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
