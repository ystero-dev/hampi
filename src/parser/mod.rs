//! Main Parser module
//!
pub use crate::tokenizer::tokenize;

use crate::error::Error;
use crate::structs::module::Asn1Module;
use crate::tokenizer::Token;

mod module;

mod utils;

mod defs;

mod types;

mod values;

mod oid;

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
        let (module, consumed) = module::parse_module(&tokens[total..])?;
        modules.push(module);
        total += consumed;
        if total == tokens.len() {
            break;
        }
    }
    Ok(modules)
}
