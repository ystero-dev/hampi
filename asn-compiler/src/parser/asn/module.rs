//! ASN.1 Module Parsing functionality
use std::collections::HashMap;

use crate::error::Error;
use crate::tokenizer::Token;

use crate::parser::utils::{expect_keyword, expect_one_of_keywords, expect_token};

use super::{
    defs::parse_definition,
    oid::parse_object_identifier,
    structs::{
        defs::Asn1Definition,
        module::{Asn1Module, Asn1ModuleName, Asn1ModuleTag},
        oid::ObjectIdentifier,
    },
};

impl Asn1Module {
    pub(crate) fn resolve_object_classes(
        &mut self,
        object_classes: &HashMap<String, Asn1Definition>,
    ) -> Result<(), Error> {
        for def in self.definitions.values_mut() {
            if !def.is_object_or_object_set() {
                continue;
            }
            if let Some(class) = def.get_object_class() {
                let classdef = object_classes.get(&class);
                if classdef.is_none() {
                    return Err(parse_error!(
                        "Error Resolving Class '{}' for Definition '{}'",
                        class,
                        def.id()
                    ));
                }
                let class = classdef.unwrap();
                def.resolve_object_class(class)?;
            }
        }
        Ok(())
    }
}

pub(in crate::parser) fn parse_module(tokens: &[Token]) -> Result<(Asn1Module, usize), Error>
where
{
    let mut consumed = 0;

    // Module Name and Object Identifier
    let (name, name_consumed) = parse_module_name(&tokens[consumed..])?;
    consumed += name_consumed;

    // DEFINITIONS Keywords
    if expect_keyword(&tokens[consumed..], "DEFINITIONS")? {
        consumed += 1;
    } else {
        return Err(unexpected_token!("DEFINITIONS", tokens[consumed]));
    }

    let (tags, tags_consumed) = maybe_parse_header_tags(&tokens[consumed..])?;
    consumed += tags_consumed;

    // FIXME: Handle EXTENSIBILITY

    if expect_token(&tokens[consumed..], Token::is_assignment)? {
        consumed += 1;
    } else {
        return Err(unexpected_token!("::=", tokens[consumed]));
    }
    if expect_keyword(&tokens[consumed..], "BEGIN")? {
        consumed += 1;
    } else {
        return Err(unexpected_token!("BEGIN", tokens[consumed]));
    }

    let (imports, imports_consumed) = parse_module_imports(&tokens[consumed..])?;
    consumed += imports_consumed;

    let mut definitions = HashMap::new();
    while !expect_keyword(&tokens[consumed..], "END")? {
        let (def, definition_consumed) = parse_definition(&tokens[consumed..])?;
        definitions.insert(def.id(), def);
        consumed += definition_consumed;
    }

    // Comes out of the loop when END is found.
    // If 'END' was never found we'd Error out at above 'while'
    consumed += 1;

    let module = Asn1Module::default()
        .name(name)
        .tags(tags)
        .imports(imports)
        .definitions(definitions);
    Ok((module, consumed))
}

fn parse_module_imports(
    tokens: &[Token],
) -> Result<(HashMap<String, Asn1ModuleName>, usize), Error> {
    let mut consumed = 0;

    // FIXME: Parse IMPORTS and EXPORTS
    let mut imports = HashMap::new();
    if expect_keyword(&tokens[consumed..], "IMPORTS")? {
        consumed += 1;

        loop {
            let mut imported_defs = vec![];
            while !expect_keyword(&tokens[consumed..], "FROM")? {
                if expect_token(&tokens[consumed..], Token::is_identifier)? {
                    let definition = tokens[consumed].text.clone();
                    imported_defs.push(definition);
                }
                consumed += 1;
                if expect_token(&tokens[consumed..], Token::is_comma)? {
                    consumed += 1;
                }
            }
            consumed += 1;
            let (module_name, module_name_consumed) = parse_module_name(&tokens[consumed..])?;
            consumed += module_name_consumed;

            for d in imported_defs {
                if imports.contains_key(&d) {
                    return Err(parse_error!("Definition '{}' is imported twice", d));
                }
                let _ = imports.insert(d, module_name.clone());
            }

            if expect_token(&tokens[consumed..], Token::is_semicolon)? {
                consumed += 1;
                break;
            }
        }
    }

    Ok((imports, consumed))
}

fn maybe_parse_header_tags(tokens: &[Token]) -> Result<(Asn1ModuleTag, usize), Error> {
    let mut consumed = 0;

    let tag =
        if expect_one_of_keywords(&tokens[consumed..], &["EXPLICIT", "IMPLICIT", "AUTOMATIC"])? {
            let tag: Asn1ModuleTag;
            match tokens[consumed].text.as_str() {
                "EXPLICIT" => tag = Asn1ModuleTag::Explicit,
                "IMPLICIT" => tag = Asn1ModuleTag::Implicit,
                "AUTOMATIC" => tag = Asn1ModuleTag::Automatic,
                _ => {
                    // Will never reach
                    return Err(parse_error!("Should Never Reach"));
                }
            }
            consumed += 1;
            if expect_keyword(&tokens[consumed..], "TAGS")? {
                consumed += 1
            } else {
                return Err(unexpected_token!("TAGS", tokens[consumed]));
            }
            tag
        } else {
            Asn1ModuleTag::Explicit
        };
    Ok((tag, consumed))
}

fn parse_module_name(tokens: &[Token]) -> Result<(Asn1ModuleName, usize), Error> {
    let mut consumed = 0;
    // First Name

    let name = if expect_token(&tokens[consumed..], Token::is_module_reference)? {
        tokens[consumed].text.clone()
    } else {
        return Err(parse_error!(
            "Module Name '{}' is not a valid Module Reference",
            tokens[consumed].text
        ));
    };
    consumed += 1;

    // Now OID
    // Optional Object Identifier
    let (oid, oid_consumed) = maybe_parse_object_identifer(&tokens[consumed..])?;
    consumed += oid_consumed;

    Ok((Asn1ModuleName::new(name, oid), consumed))
}

fn maybe_parse_object_identifer(
    tokens: &[Token],
) -> Result<(Option<ObjectIdentifier>, usize), Error> {
    match expect_token(tokens, Token::is_curly_begin) {
        Ok(success) => {
            if success {
                match parse_object_identifier(tokens) {
                    Ok((oid, consumed)) => Ok((Some(oid), consumed)),
                    Err(e) => Err(e),
                }
            } else {
                Ok((None, 0))
            }
        }
        // This can only `Err` when we've reached End of tokens, which is a `None` object
        // Identifier.
        Err(_) => Ok((None, 0)),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn empty_module_success() {
        let input = "ModuleFoo DEFINITIONS ::= BEGIN END";
        let reader = std::io::BufReader::new(std::io::Cursor::new(input));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());

        let mut tokens = tokens.unwrap();
        let module = parse_module(&mut tokens);
        assert!(module.is_ok(), "{}: {:#?}", input, module.err());

        let (module, consumed) = module.unwrap();
        assert_eq!(consumed, 5);

        assert!(module.definitions.is_empty());
        assert!(module.imports.is_empty());
        assert_eq!(module.tags, Asn1ModuleTag::Explicit);
    }
    // TODO: Test Cases for imports (count), Tags (type), Definitions (count))
    // TODO: Test Cases for missing BEGIN, END, DEFINITIONS, ::=

    #[test]
    fn parse_module_name_tests() {
        struct ParseModuleNameTestCase<'tc> {
            input: &'tc str,
            success: bool,
            consumed: usize,
            oid_present: bool,
        }

        let test_cases = vec![
            ParseModuleNameTestCase {
                input: "ModuleFoo",
                success: true,
                consumed: 1,
                oid_present: false,
            },
            ParseModuleNameTestCase {
                input: "moduleFoo",
                success: false,
                consumed: 0,
                oid_present: false,
            },
            ParseModuleNameTestCase {
                input: "ModuleFoo { iso }",
                success: true,
                consumed: 4,
                oid_present: true,
            },
            ParseModuleNameTestCase {
                input: "ModuleFoo { iso ",
                success: false,
                consumed: 0,
                oid_present: false,
            },
            ParseModuleNameTestCase {
                input: "ModuleFoo iso ", // This is a success, 'iso' is ignored.
                success: true,
                consumed: 1,
                oid_present: false,
            },
            ParseModuleNameTestCase {
                input: "NGAP-CommonDataTypes { itu-t (0) identified-organization (4) etsi (0) mobileDomain (0) ngran-Access (22) modules (3) ngap (1) version1 (1) ngap-CommonDataTypes (3) }", success: true, consumed: 39, oid_present: true,
            }
        ];

        for tc in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(tc.input));
            let tokens = tokenize(reader);
            assert!(tokens.is_ok());

            let mut tokens = tokens.unwrap();
            let module = parse_module_name(&mut tokens);
            assert_eq!(module.is_ok(), tc.success, "{}", tc.input);

            if tc.success {
                let (module, consumed) = module.unwrap();
                assert_eq!(consumed, tc.consumed);
                assert_eq!(module.oid.is_some(), tc.oid_present);
            }
        }
    }
}
