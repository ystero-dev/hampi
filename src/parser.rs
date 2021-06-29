//! Main Parser module
//!
pub use crate::tokenizer::tokenize;

use crate::error::Error;
use crate::oid::{parse_object_identifier, ObjectIdentifier};
use crate::structs::Asn1Module;
use crate::tokenizer::Token;

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
        return Err(Error::ParseError);
    }
    let idx = curly_end.unwrap();

    let oid = parse_object_identifier(&tokens[..=idx])?;

    Ok((Some(oid), idx + 1))
}

fn parse_module_name<'parser>(token: &'parser Token) -> Result<String, Error> {
    if token.is_identifier() && token.text.starts_with(char::is_uppercase) {
        Ok(token.text.clone())
    } else {
        Err(Error::ParseError)
    }
}

fn parse_internal<'parser>(tokens: &'parser [Token]) -> Result<(Asn1Module, usize), Error>
where
{
    let mut consumed = 0;

    // Module Name first
    let name = parse_module_name(&tokens[consumed])?;
    consumed += 1;

    // Optional Object Identifier
    let (oid, oid_consumed) = maybe_parse_object_identifer(&tokens[consumed..])?;
    consumed += oid_consumed;

    let module = Asn1Module::empty(&name).oid(oid);
    Ok((module, consumed))
}

/// Parse the tokens into internal Asn1Module representation
///
/// Token obtained from running [`tokenize`][`crate::tokenizer::tokenize] on an ANS file are parsed
/// into an internal representation of [`Asn1Module`][`crate::structs::Asn1Module`]. Semantic
/// errors during parsing the tokens are returned as `ParseError`.
pub fn parse<'parser>(tokens: &'parser mut Vec<Token>) -> Result<Vec<Asn1Module>, Error> {
    // Get rid of the comments, it complicates things
    tokens.retain(|x| !x.is_comment());

    let mut modules = vec![];
    let mut total = 0;
    loop {
        let (module, consumed) = parse_internal(&tokens[total..])?;
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
    fn name_empty_module() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("ModuleFoo"));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());

        let mut tokens = tokens.unwrap();
        let module = parse(&mut tokens);
        assert!(module.is_ok(), "{:#?}", module.err());
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
    fn name_empty_oid() {
        let reader = std::io::BufReader::new(std::io::Cursor::new("ModuleFoo { iso } "));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());

        let mut tokens = tokens.unwrap();
        let module = parse(&mut tokens);
        assert!(module.is_ok(), "{:#?}", module.err());
        let modules = module.unwrap();
        assert!(modules.len() == 1);
    }
}
