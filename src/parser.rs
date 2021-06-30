//! Main Parser module
//!

pub use crate::tokenizer::tokenize;

use crate::error::Error;
use crate::oid::{parse_object_identifier, ObjectIdentifier};
use crate::structs::{Asn1Module, Asn1ModuleTag};
use crate::tokenizer::Token;

// Required by `expect_*` functions
type TokenChecker = fn(&Token) -> bool;

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

fn parse_module_name<'parser>(token: &'parser Token) -> Result<String, Error> {
    if token.is_identifier() && token.text.starts_with(char::is_uppercase) {
        Ok(token.text.clone())
    } else {
        Err(Error::ParseError(format!(
            "Module Name '{}' should start with a Capital Letter.",
            token.text
        )))
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

    // DEFINITIONS Keywords
    if expect_keyword(&tokens[consumed..], "DEFINITIONS")? {
        consumed += 1;
    } else {
        return Err(Error::UnexpectedToken(
            "DEFINITIONS".to_string(),
            tokens[consumed].clone(),
        ));
    }

    let tags =
        if expect_one_of_keywords(&tokens[consumed..], &["EXPLICIT", "IMPLICIT", "AUTOMATIC"])? {
            let tag: Asn1ModuleTag;
            match tokens[consumed].text.as_str() {
                "EXPLICIT" => tag = Asn1ModuleTag::Explicit,
                "IMPLICIT" => tag = Asn1ModuleTag::Implicit,
                "AUTOMATIC" => tag = Asn1ModuleTag::Automatic,
                _ => {
                    // Will never reach
                    return Err(Error::ParseError("Should Never Reach".to_string()));
                }
            }
            consumed += 1;
            if expect_keyword(&tokens[consumed..], "TAGS")? {
                consumed += 1
            } else {
                return Err(Error::UnexpectedToken(
                    "TAGS".to_string(),
                    tokens[consumed].clone(),
                ));
            }
            tag
        } else {
            Asn1ModuleTag::Explicit
        };
    if expect_token(&tokens[consumed..], Token::is_assignment)? {
        consumed += 1;
    } else {
        return Err(Error::UnexpectedToken(
            "::=".to_string(),
            tokens[consumed].clone(),
        ));
    }
    if expect_keyword(&tokens[consumed..], "BEGIN")? {
        consumed += 1;
    }
    while !expect_keyword(&tokens[consumed..], "END")? {
        consumed += 1;
    }

    // Comes out of the loop when END is found. FIXME: If we never have END?
    consumed += 1;

    let module = Asn1Module::empty(&name).oid(oid).tags(tags);
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

pub(crate) fn expect_keyword<'parser>(
    tokens: &'parser [Token],
    keyword: &str,
) -> Result<bool, Error> {
    if tokens.len() == 0 {
        Err(Error::UnexpectedEndOfTokens)
    } else {
        Ok(tokens[0].is_keyword() && tokens[0].text == keyword)
    }
}

pub(crate) fn expect_one_of_keywords<'parser>(
    tokens: &'parser [Token],
    keywords: &[&str],
) -> Result<bool, Error> {
    if tokens.len() == 0 {
        Err(Error::UnexpectedEndOfTokens)
    } else {
        Ok(keywords.iter().any(|&k| expect_keyword(tokens, k).unwrap()))
    }
}

pub(crate) fn expect_token<'parser>(
    tokens: &'parser [Token],
    checker: TokenChecker,
) -> Result<bool, Error> {
    if tokens.len() == 0 {
        Err(Error::UnexpectedEndOfTokens)
    } else {
        Ok(checker(&tokens[0]))
    }
}

pub(crate) fn expect_one_of_tokens<'parser>(
    tokens: &'parser [Token],
    checkers: &'parser [TokenChecker],
) -> Result<bool, Error> {
    if tokens.len() == 0 {
        Err(Error::UnexpectedEndOfTokens)
    } else {
        Ok(checkers.iter().any(|&c| expect_token(tokens, c).unwrap()))
    }
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
