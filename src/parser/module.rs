//! ASN.1 Module Parsing functionality
use std::collections::HashMap;

use crate::error::Error;
use crate::structs::{
    module::{Asn1Module, Asn1ModuleName, Asn1ModuleTag},
    oid::ObjectIdentifier,
};
use crate::tokenizer::Token;

use super::defs::parse_definition;
use super::oid::parse_object_identifier;
use super::utils::{expect_keyword, expect_one_of_keywords, expect_token};

pub(crate) fn parse_module<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Module, usize), Error>
where
{
    let mut consumed = 0;

    // Module Name and Object Identifier
    let (name, name_consumed) = parse_asn1_module_name(&tokens[consumed..])?;
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

fn parse_module_imports<'parser>(
    tokens: &'parser [Token],
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
            let (module_name, module_name_consumed) = parse_asn1_module_name(&tokens[consumed..])?;
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

fn maybe_parse_header_tags<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1ModuleTag, usize), Error> {
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

fn parse_asn1_module_name<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1ModuleName, usize), Error> {
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
        return Err(parse_error!("Expected '}}'. Never Found."));
    }
    let idx = curly_end.unwrap();

    let oid = parse_object_identifier(&tokens[..=idx])?;

    Ok((Some(oid), idx + 1))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parser::tokenize;

    #[test]
    fn empty_basic_module_success() {
        let input = "ModuleFoo DEFINITIONS ::= BEGIN END";
        let reader = std::io::BufReader::new(std::io::Cursor::new(input));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());

        let mut tokens = tokens.unwrap();
        let module = parse_module(&mut tokens);
        assert!(module.is_ok(), "{}: {:#?}", input, module.err());
    }

    #[test]
    fn name_empty_module_failure() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("ModuleFoo"));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());

        let mut tokens = tokens.unwrap();
        let module = parse_module(&mut tokens);
        assert!(module.is_err(), "{:#?}", module.ok());
    }

    #[test]
    fn name_lowercase_fail() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("moduleFoo"));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());

        let mut tokens = tokens.unwrap();
        let module = parse_module(&mut tokens);
        assert!(module.is_err(), "{:#?}", module.ok());
    }

    #[test]
    fn name_oid_failure() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("ModuleFoo { iso } "));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());

        let mut tokens = tokens.unwrap();
        let module = parse_module(&mut tokens);
        assert!(module.is_err(), "{:#?}", module.ok());
    }
}
