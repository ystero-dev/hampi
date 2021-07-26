//! Parsing related to "CHOICE" Type

use crate::error::Error;
use crate::tokenizer::Token;

use crate::parser::utils::{expect_keyword, expect_token};

use crate::parser::asn::structs::types::constructed::{Asn1TypeChoice, ChoiceAdditionGroup};

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
    let mut additions = vec![];
    let mut extension_markers = 0;
    loop {
        let (component, component_consumed) = match parse_component(&tokens[consumed..]) {
            Ok(result) => (Some(result.0), result.1),
            Err(_) => (None, 0),
        };
        if component.is_some() {
            components.push(component.unwrap());
            consumed += component_consumed;
        }

        if expect_token(&tokens[consumed..], Token::is_addition_groups_begin)? {
            if extension_markers != 1 {
                return Err(parse_error!(
                    "Addition Component can only be present after an Extension Marker!"
                ));
            } else {
                let (addition_group, addition_group_consumed) =
                    parse_choice_addition_group(&tokens[consumed..])?;
                additions.push(addition_group);
                consumed += addition_group_consumed;
            }
        }

        if expect_token(&tokens[consumed..], Token::is_comma)? {
            consumed += 1;
        }

        // Consume extension marker and next comma if present
        if expect_token(&tokens[consumed..], Token::is_extension)? {
            extension_markers += 1;
            if extension_markers > 1 {
                return Err(parse_error!(
                    "Only one Extension Marker is allowed in a 'CHOICE')"
                ));
            }
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
        Asn1TypeChoice {
            components,
            additions,
        },
        consumed,
    ))
}

fn parse_choice_addition_group<'parser>(
    tokens: &'parser [Token],
) -> Result<(ChoiceAdditionGroup, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_addition_groups_begin)? {
        return Err(unexpected_token!("'[['", tokens[consumed]));
    }
    consumed += 1;

    let version = if expect_token(&tokens[consumed..], Token::is_numeric)? {
        let version = tokens[consumed].text.clone();
        consumed += 1;
        if !expect_token(&tokens[consumed..], Token::is_colon)? {
            return Err(unexpected_token!("':'", tokens[consumed]));
        }
        consumed += 1;
        Some(version)
    } else {
        None
    };

    let mut components = vec![];
    loop {
        let (component, component_consumed) = match parse_component(&tokens[consumed..]) {
            Ok(result) => (Some(result.0), result.1),
            Err(_) => (None, 0),
        };

        if component.is_some() {
            components.push(component.unwrap());
        }
        consumed += component_consumed;

        if expect_token(&tokens[consumed..], Token::is_comma)? {
            consumed += 1;
        }

        if expect_token(&tokens[consumed..], Token::is_addition_groups_end)? {
            consumed += 1;
            break;
        }
    }
    if components.is_empty() {
        Err(parse_error!("Additional Components cannot be empty!"))
    } else {
        Ok((
            ChoiceAdditionGroup {
                version,
                components,
            },
            consumed,
        ))
    }
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
            addition_components_count: usize,
            tokens_consumed: usize,
        }

        let test_cases = vec![
            ParseChoiceTestCase {
                input: "CHOICE { a BOOL, b INTEGER} ",
                success: true,
                components_count: 2,
                addition_components_count: 0,
                tokens_consumed: 8,
            },
            ParseChoiceTestCase {
                input: "CHOICE { a BOOL, b INTEGER, c CHOICE { d INTEGER, e NULL} } ",
                success: true,
                components_count: 3,
                addition_components_count: 0,
                tokens_consumed: 18,
            },
            ParseChoiceTestCase {
                input: "CHOICE { local INTEGER (0..65535), global OBJECT IDENTIFIER }",
                success: true,
                components_count: 2,
                addition_components_count: 0,
                tokens_consumed: 14,
            },
            ParseChoiceTestCase {
                input: "CHOICE { a INTEGER , b Enum, ..., [[ c CHOICE { d INTEGER } ]] }",
                success: true,
                components_count: 2,
                addition_components_count: 1,
                tokens_consumed: 19,
            },
            ParseChoiceTestCase {
                input: "CHOICE { a INTEGER , b Enum, [[ c CHOICE { d INTEGER } ]] }",
                success: false,
                components_count: 0,
                addition_components_count: 0,
                tokens_consumed: 0,
            },
            ParseChoiceTestCase {
                input: "CHOICE { a INTEGER , b Enum, ..., [[ ]] }",
                success: false,
                components_count: 0,
                addition_components_count: 0,
                tokens_consumed: 0,
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
                assert_eq!(
                    choice.additions.len(),
                    tc.addition_components_count,
                    "{}",
                    tc.input
                );
                assert_eq!(choice_consumed, tc.tokens_consumed, "{}", tc.input);
            }
        }
    }
}
