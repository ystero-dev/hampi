//! Utility functions for Parsing Constructed types

use crate::error::Error;
use crate::parser::{types::parse_type, utils::expect_token};
use crate::tokenizer::Token;

use crate::structs::constructed::Component;

pub(crate) fn parse_component<'parser>(
    tokens: &'parser [Token],
) -> Result<(Component, usize), Error> {
    let mut consumed = 0;
    if !expect_token(&tokens[consumed..], Token::is_value_reference)? {
        return Err(unexpected_token!("'IDENTIFIER'", tokens[consumed]));
    }
    let id = tokens[consumed].text.clone();
    consumed += 1;

    let (ty, ty_consumed) = parse_type(&tokens[consumed..])?;
    consumed += ty_consumed;

    Ok((Component { id, ty }, consumed))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn parse_component_test() {
        struct ParseComponentTestCase<'tc> {
            input: &'tc str,
            success: bool,
            consumed: usize,
        }
        let test_cases = vec![
            ParseComponentTestCase {
                input: "local INTEGER (0..65535)",
                success: true,
                consumed: 7,
            },
            ParseComponentTestCase {
                input: "global                          OBJECT IDENTIFIER",
                success: true,
                consumed: 2,
            },
        ];
        for tc in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(tc.input));
            let tokens = tokenize(reader);
            assert!(tokens.is_ok());
            let tokens = tokens.unwrap();

            let component = parse_component(&tokens);
            assert_eq!(component.is_ok(), tc.success, "{}", tc.input);
        }
    }
}
