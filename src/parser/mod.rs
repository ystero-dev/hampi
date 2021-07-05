//! Main Parser module
//!
use std::collections::HashMap;

pub use crate::tokenizer::tokenize;

use crate::error::Error;
use crate::structs::module::Asn1Module;
use crate::tokenizer::Token;

// Internal module structures
mod name;

mod utils;

mod defs;

mod types;

mod values;

use defs::parse_definition;
use name::parse_asn1_module_name;
use types::parse_type;
use utils::{expect_keyword, expect_token, expect_token_one_of, expect_tokens, maybe_parse_tags};
use values::parse_value;

fn parse_module<'parser>(tokens: &'parser [Token]) -> Result<(Asn1Module, usize), Error>
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

    let (tags, tags_consumed) = maybe_parse_tags(&tokens[consumed..])?;
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

/// Parse the tokens into internal Asn1Module representation
///
/// Token obtained from running [`tokenize`][`crate::tokenizer::tokenize] on an ANS file are parsed
/// into an internal representation of [`Asn1Module`][`crate::structs::Asn1Module`]. Semantic
/// errors during parsing the tokens are returned as appropriate variant of `Error`.
pub fn parse<'parser>(tokens: &'parser mut Vec<Token>) -> Result<Vec<Asn1Module>, Error> {
    // Get rid of the comments, it complicates things
    tokens.retain(|x| !x.is_comment());

    let mut modules = vec![];
    let mut total = 0;
    loop {
        let (module, consumed) = parse_module(&tokens[total..])?;
        modules.push(module);
        total += consumed;
        if total == tokens.len() {
            break;
        }
    }
    Ok(modules)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty_basic_module_success() {
        let input = "ModuleFoo DEFINITIONS ::= BEGIN END";
        let reader = std::io::BufReader::new(std::io::Cursor::new(input));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());

        let mut tokens = tokens.unwrap();
        let module = parse(&mut tokens);
        assert!(module.is_ok(), "{}: {:#?}", input, module.err());
    }

    #[test]
    fn name_empty_module_failure() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("ModuleFoo"));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());

        let mut tokens = tokens.unwrap();
        let module = parse(&mut tokens);
        assert!(module.is_err(), "{:#?}", module.ok());
    }

    #[test]
    fn name_lowercase_fail() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("moduleFoo"));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());

        let mut tokens = tokens.unwrap();
        let module = parse(&mut tokens);
        assert!(module.is_err(), "{:#?}", module.ok());
    }

    #[test]
    fn name_empty_oid_failure() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("ModuleFoo { iso } "));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());

        let mut tokens = tokens.unwrap();
        let module = parse(&mut tokens);
        assert!(module.is_err(), "{:#?}", module.ok());
    }
}
