//! Parsing functionality related to Object Identifier

use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::error::Error;
use crate::structs::oid::{OIDComponent, ObjectIdentifier};
use crate::tokenizer::Token;

use super::utils::{expect_one_of_tokens, expect_token, expect_tokens};

lazy_static! {
    static ref WELL_KNOWN_OID_NAMES: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("iso", 1);
        m.insert("itu-t", 0);
        m.insert("ccitt", 0);
        m.insert("joint-iso-itu-t", 2);
        m.insert("joint-iso-ccitt", 2);
        // FIXME: We use the itu-t identified-organization (iso standard-orgainization is 3)
        m.insert("identified-organization", 4);
        m.insert("network-operator", 3);
        m.insert("administration", 2);
        m.insert("question", 1);
        m
    };
}

// Parses a named OID component
//
// Parses named OID components of the form `iso` or `iso(1)`
fn parse_named_oid_component<'parser>(
    tokens: &'parser [Token],
) -> Result<(OIDComponent, usize), Error> {
    if !expect_token(&tokens, Token::is_value_reference)? {
        return Err(unexpected_token!("'IDENTIFIER'", tokens[0]));
    }
    let name_token = &tokens[0];
    let name = &name_token.text;
    let (number, consumed) = match expect_tokens(
        &tokens[1..],
        &[
            &[Token::is_round_begin],
            &[Token::is_numeric],
            &[Token::is_round_end],
        ],
    ) {
        Ok(success) => {
            if success {
                let number_token = &tokens[2];
                let number = number_token
                    .text
                    .parse::<u32>()
                    .map_err(|_| invalid_token!(number_token))?;
                (number, 4)
            } else {
                let number = WELL_KNOWN_OID_NAMES.get(name.as_str());
                if number.is_none() {
                    return Err(unknown_oid_name!(name_token));
                }
                (*number.unwrap(), 1)
            }
        }
        Err(_) => {
            let number = WELL_KNOWN_OID_NAMES.get(name.as_str());
            if number.is_none() {
                return Err(unknown_oid_name!(name_token));
            }
            (*number.unwrap(), 1)
        }
    };

    Ok((OIDComponent::new(Some(name.clone()), number), consumed))
}

// Wrapper for Parsing an OID Component
//
// Parses Either Numbered or Named/Numbered OID components
fn parse_oid_component<'parser>(tokens: &'parser [Token]) -> Result<(OIDComponent, usize), Error> {
    let consumed = 0;

    if expect_one_of_tokens(
        &tokens[consumed..],
        &[Token::is_identifier, Token::is_numeric],
    )? {
        let first = &tokens[0];
        if first.is_identifier() {
            parse_named_oid_component(tokens)
        } else {
            let number = first
                .text
                .parse::<u32>()
                .map_err(|_| invalid_token!(first))?;
            Ok((OIDComponent::new(None, number), 1))
        }
    } else {
        Err(unexpected_token!(
            "Expected 'identifier' or 'number'",
            tokens[0]
        ))
    }
}

pub(super) fn parse_object_identifier<'parser>(
    tokens: &'parser [Token],
) -> Result<(ObjectIdentifier, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("{", tokens[consumed]));
    }
    consumed += 1;

    let mut components = vec![];
    while !expect_token(&tokens[consumed..], Token::is_curly_end)? {
        let (component, component_consumed) = parse_oid_component(&tokens[consumed..])?;
        components.push(component);
        consumed += component_consumed;
    }
    consumed += 1;

    // FIXME: OID with empty components is an Error?

    Ok((ObjectIdentifier::new(components), consumed))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokenizer::tokenize;

    struct OIDTestCase<'testcase> {
        input: &'testcase str,
        success: bool,
        consumed: usize,
        parse_component_only: bool,
        components_count: usize,
    }

    #[test]
    fn object_identifier_cases() {
        let test_cases = vec![
            OIDTestCase {
                input: " iso ",
                success: true,
                consumed: 1,
                parse_component_only: true,
                components_count: 0,
            },
            OIDTestCase {
                input: " foo ",
                success: false,
                consumed: 1,
                parse_component_only: true,
                components_count: 0,
            },
            OIDTestCase {
                input: " something(3) ",
                success: true,
                consumed: 4,
                parse_component_only: true,
                components_count: 0,
            },
            OIDTestCase {
                input: " something(-3) ",
                success: false,
                consumed: 4,
                parse_component_only: true,
                components_count: 0,
            },
            OIDTestCase {
                input: " iso() ",
                success: true,
                consumed: 1,
                parse_component_only: true,
                components_count: 0,
            },
            OIDTestCase {
                input: "{ iso() }",
                success: false,
                consumed: 4,
                parse_component_only: false,
                components_count: 0,
            },
            OIDTestCase {
                input: "{ iso(1 }",
                success: false,
                consumed: 4,
                parse_component_only: false,
                components_count: 0,
            },
            OIDTestCase {
                input: " { iso something(3) }",
                success: true,
                consumed: 7,
                parse_component_only: false,
                components_count: 2,
            },
            OIDTestCase {
                input: "{ iso }",
                success: true,
                consumed: 3,
                parse_component_only: false,
                components_count: 1,
            },
        ];

        for tc in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(tc.input));
            let tokens = tokenize(reader);
            assert!(tokens.is_ok());

            let tokens = tokens.unwrap();
            if tc.parse_component_only {
                let oidcomp = parse_oid_component(&tokens);
                assert_eq!(oidcomp.is_ok(), tc.success, "{:#?}", tc.input);
                if tc.success {
                    let (_oidcomp, consumed) = oidcomp.unwrap();
                    assert_eq!(consumed, tc.consumed, "{:#?}", tc.input);
                }
            } else {
                let oid = parse_object_identifier(&tokens);
                assert_eq!(oid.is_ok(), tc.success, "{:#?}", tc.input);
                if tc.success {
                    let (oid, consumed) = oid.unwrap();
                    assert_eq!(oid.len(), tc.components_count);
                    assert_eq!(consumed, tc.consumed);
                }
            }
        }
    }
}
