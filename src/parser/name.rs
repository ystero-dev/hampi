//! Structures and functions related to handling ASN.1 Module Names
//!
//! An ASN.1 Module name is an Identifier with optional 'OBJECT IDENTIFIER'. It is better that the
//! module name is treated as such since this is required at several places (eg. while parsing
//! Import definitions etc.)
//!
use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::error::Error;
use crate::parser::expect_token;
use crate::structs::{Asn1ModuleName, OIDComponent, ObjectIdentifier};
use crate::tokenizer::Token;

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
    if tokens.len() == 0 {
        return Err(Error::UnexpectedEndOfTokens);
    }

    if tokens.len() < 4 {
        let token = &tokens[0];
        let number = WELL_KNOWN_OID_NAMES.get(token.text.as_str());
        if number.is_none() {
            return Err(Error::UnknownNamedIdentifier(token.text.clone()));
        }
        let number = *number.unwrap();
        return Ok((OIDComponent::new(Some(token.text.clone()), number), 1));
    }

    let (tok1, tok2, tok3, tok4) = (&tokens[0], &tokens[1], &tokens[2], &tokens[3]);
    if tok2.is_round_begin() && tok3.is_numeric() && tok4.is_round_end() {
        return Ok((
            OIDComponent::new(
                Some(tok1.text.clone()),
                tok3.text
                    .parse::<u32>()
                    .map_err(|_| Error::InvalidToken(tok3.clone()))?,
            ),
            4,
        ));
    } else {
        let number = WELL_KNOWN_OID_NAMES.get(tok1.text.as_str());
        if number.is_none() {
            return Err(Error::UnknownNamedIdentifier(tok1.text.clone()));
        }
        let number = *number.unwrap();
        return Ok((OIDComponent::new(Some(tok1.text.clone()), number), 1));
    }
}

// Wrapper for Parsing an OID Component
//
// Parses Either Numbered or Named/Numbered OID components
fn parse_oid_component<'parser>(tokens: &'parser [Token]) -> Result<(OIDComponent, usize), Error> {
    if tokens.len() == 0 {
        return Err(Error::UnexpectedEndOfTokens);
    }

    let first = &tokens[0];
    if first.is_identifier() {
        parse_named_oid_component(tokens)
    } else if first.is_numeric() {
        let number = first
            .text
            .parse::<u32>()
            .map_err(|_| Error::InvalidToken(first.clone()))?;
        Ok((OIDComponent::new(None, number), 1))
    } else {
        Err(Error::UnexpectedToken(
            "Expected 'identifier' or 'number'".to_string(),
            first.clone(),
        ))
    }
}

pub(crate) fn parse_object_identifier<'parser>(
    tokens: &'parser [Token],
) -> Result<ObjectIdentifier, Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(Error::UnexpectedToken(
            "{".to_string(),
            tokens[consumed].clone(),
        ));
    }
    consumed += 1;

    let mut components = vec![];
    loop {
        let result = parse_oid_component(&tokens[consumed..]);
        match result {
            Ok((comp, c)) => {
                components.push(comp);
                consumed += c;
            }
            Err(e) => {
                let tok = &tokens[consumed];
                if tok.is_curly_end() {
                    break;
                } else {
                    return Err(e);
                }
            }
        }
    }
    Ok(ObjectIdentifier::new(components))
}

pub(super) fn parse_asn1_module_name<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1ModuleName, usize), Error> {
    let mut consumed = 0;
    // First Name

    let name = if expect_token(&tokens[consumed..], Token::is_module_reference)? {
        tokens[consumed].text.clone()
    } else {
        return Err(Error::ParseError(format!(
            "Module Name '{}' is not a valid Module Reference",
            tokens[consumed].text
        )));
    };
    consumed += 1;

    // Now OID
    // Optional Object Identifier
    let (oid, oid_consumed) = maybe_parse_object_identifer(&tokens[consumed..])?;
    consumed += oid_consumed;

    Ok((Asn1ModuleName::new(name, oid), consumed))
}

fn maybe_parse_object_identifer<'parser>(
    tokens: &'parser [Token],
) -> Result<(Option<ObjectIdentifier>, usize), Error> {
    if tokens.is_empty() || !tokens[0].is_curly_begin() {
        return Ok((None, 0));
    }
    let mut curly_end: Option<usize> = None;
    for (idx, token) in tokens.iter().enumerate() {
        if token.is_curly_end() {
            curly_end = Some(idx);
            break;
        }
    }

    if curly_end.is_none() {
        return Err(Error::ParseError("Expected '}'. Never Found.".to_string()));
    }
    let idx = curly_end.unwrap();

    let oid = parse_object_identifier(&tokens[..=idx])?;

    Ok((Some(oid), idx + 1))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parser::tokenize;

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
                input: "{ iso }",
                success: true,
                consumed: 0,
                parse_component_only: false,
                components_count: 1,
            },
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
                consumed: 0,
                parse_component_only: false,
                components_count: 2,
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
                    let oid = oid.unwrap();
                    assert_eq!(oid.len(), tc.components_count);
                }
            }
        }
    }
}
