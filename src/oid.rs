//! Structures and functions related to handling ASN.1 Object Identifier
use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::error::Error;
use crate::tokenizer::{expect_token, Token};

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

struct OIDComponent {
    name: Option<String>,
    number: u32,
}

impl std::fmt::Display for OIDComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name.is_some() {
            write!(f, "{}({})", self.name.as_ref().unwrap(), self.number)
        } else {
            write!(f, "{}", self.number)
        }
    }
}

impl std::fmt::Debug for OIDComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
#[derive(Default)]
pub struct ObjectIdentifier {
    components: Vec<OIDComponent>,
}

impl std::fmt::Display for ObjectIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((first, rest)) = self.components.split_first() {
            write!(f, "{}", first)?;
            for c in rest {
                write!(f, ".{}", c)?;
            }
        } else {
            write!(f, "EMPTY")?;
        }
        write!(f, "")
    }
}

impl std::fmt::Debug for ObjectIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

// Parses a named OID component
//
// Parses named OID components of the form `iso` or `iso(1)`
fn parse_named_oid_component<'parser>(
    tokens: &'parser [Token],
) -> Result<(OIDComponent, usize), Error> {
    if tokens.len() == 0 {
        return Err(Error::ParseError);
    }

    if tokens.len() < 4 {
        let token = &tokens[0];
        let number = WELL_KNOWN_OID_NAMES.get(token.text.as_str());
        if number.is_none() {
            return Err(Error::ParseError);
        }
        let number = *number.unwrap();
        return Ok((
            OIDComponent {
                name: Some(token.text.clone()),
                number,
            },
            1,
        ));
    }

    let (tok1, tok2, tok3, tok4) = (&tokens[0], &tokens[1], &tokens[2], &tokens[3]);
    if tok2.is_round_begin() && tok3.is_numeric() && tok4.is_round_end() {
        return Ok((
            OIDComponent {
                name: Some(tok1.text.clone()),
                number: tok3.text.parse::<u32>().map_err(|_| Error::ParseError)?,
            },
            4,
        ));
    } else {
        let number = WELL_KNOWN_OID_NAMES.get(tok1.text.as_str());
        if number.is_none() {
            return Err(Error::ParseError);
        }
        let number = *number.unwrap();
        return Ok((
            OIDComponent {
                name: Some(tok1.text.clone()),
                number,
            },
            1,
        ));
    }
}

// Wrapper for Parsing an OID Component
//
// Parses Either Numbered or Named/Numbered OID components
fn parse_oid_component<'parser>(tokens: &'parser [Token]) -> Result<(OIDComponent, usize), Error> {
    if tokens.len() == 0 {
        return Err(Error::ParseError);
    }

    let first = &tokens[0];
    if first.is_identifier() {
        parse_named_oid_component(tokens)
    } else if first.is_numeric() {
        let number = first.text.parse::<u32>().map_err(|_| Error::ParseError)?;
        Ok((OIDComponent { name: None, number }, 1))
    } else {
        Err(Error::ParseError)
    }
}

pub(crate) fn parse_object_identifier<'parser>(
    tokens: &'parser [Token],
) -> Result<ObjectIdentifier, Error> {
    let t: &Token;
    let mut consumed = 0;

    t = &tokens[consumed];
    if !expect_token(t, Token::is_curly_begin) {
        return Err(Error::ParseError);
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
    Ok(ObjectIdentifier { components })
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
                    assert_eq!(oid.components.len(), tc.components_count);
                }
            }
        }
    }
}
