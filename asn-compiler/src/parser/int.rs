//! 'parser' Inernal module, API functions from this module are exported.

use crate::error::Error;
use crate::tokenizer::Token;

use crate::parser::asn::structs::module::Asn1Module;

use super::asn::parse_module;

/// Parse the tokens into internal Asn1Module representation
///
/// Token obtained from running [`tokenize`][`crate::tokenizer::tokenize] on an ANS file are parsed
/// into an internal representation of [`Asn1Module`][`crate::structs::Asn1Module`]. Semantic
/// errors during parsing the tokens are returned as appropriate variant of `Error`.
pub fn parse(tokens: &mut Vec<Token>) -> Result<Vec<Asn1Module>, Error> {
    // Get rid of the comments, it complicates things
    tokens.retain(|x| !x.is_comment());

    let mut modules = vec![];
    let mut total = 0;
    loop {
        let (module, consumed) = parse_module(&tokens[total..])?;
        log::trace!(
            "Module '{}' parsed, adding to the list of modules to be compiled...",
            module.get_module_name()
        );
        modules.push(module);
        total += consumed;
        if total == tokens.len() {
            break;
        }
    }
    Ok(modules)
}

// TODO: Test cases, at-least single-module, multiple modules etc.
