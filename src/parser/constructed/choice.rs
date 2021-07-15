//! Parsing related to "CHOICE" Type

use crate::error::Error;
use crate::parser::utils::{expect_keyword, expect_token};
use crate::structs::constructed::Asn1TypeChoice;
use crate::tokenizer::Token;

use super::utils::parse_component;

// Parse a Choice Type
//
// The current implementation supports a very simple choice definition, where, everything is dumped
// into 'root' components. Additional extension components or version groups etc. are not supported
// for now. May be supported later if needed.
pub(crate) fn parse_choice_type<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1TypeChoice, usize), Error> {
    let mut consumed = 0;

    if !expect_keyword(&tokens[consumed..], "CHOICE")? {
        return Err(unexpected_token!("'CHOICE'", tokens[consumed]));
    }
    consumed += 1;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'CHOICE'", tokens[consumed]));
    }
    consumed += 1;

    let mut components = vec![];
    loop {
        let (component, component_consumed) = parse_component(&tokens[consumed..])?;
        components.push(component);
        consumed += component_consumed;

        if expect_token(&tokens[consumed..], Token::is_comma)? {
            consumed += 1;
        }

        // Consume extension marker and next comma if present
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

    Ok((Asn1TypeChoice { components }, consumed))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn parse_choice_type_tests() {
        struct ParseChoiceTestCase<'tc> {
            input: &'tc str,
            success: bool,
            components_count: usize,
            tokens_consumed: usize,
        }

        let test_cases = vec![
            ParseChoiceTestCase {
                input: "CHOICE { a BOOL, b INTEGER} ",
                success: true,
                components_count: 2,
                tokens_consumed: 8,
            },
            ParseChoiceTestCase {
                input: "CHOICE { a BOOL, b INTEGER, c CHOICE { d INTEGER, e NULL} } ",
                success: true,
                components_count: 3,
                tokens_consumed: 18,
            },
        ];

        for tc in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(tc.input));
            let tokens = tokenize(reader);
            assert!(tokens.is_ok());
            let tokens = tokens.unwrap();

            let choice = parse_choice_type(&tokens);
            assert_eq!(choice.is_ok(), tc.success, "{}", tc.input);

            if tc.success {
                let (choice, choice_consumed) = choice.unwrap();
                assert_eq!(choice.components.len(), tc.components_count, "{}", tc.input);
                assert_eq!(choice_consumed, tc.tokens_consumed, "{}", tc.input);
            }
        }
    }
}
