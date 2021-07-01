//! Utility functions for the parser module

use crate::error::Error;
use crate::structs::Asn1ModuleTag;
use crate::tokenizer::Token;

// Required by `expect_*` functions
type TokenChecker = fn(&Token) -> bool;

pub(crate) fn expect_keyword<'parser>(
    tokens: &'parser [Token],
    keyword: &str,
) -> Result<bool, Error> {
    if tokens.len() == 0 {
        Err(unexpected_end!())
    } else {
        Ok(tokens[0].is_keyword() && tokens[0].text == keyword)
    }
}

pub(crate) fn expect_one_of_keywords<'parser>(
    tokens: &'parser [Token],
    keywords: &[&str],
) -> Result<bool, Error> {
    if tokens.len() == 0 {
        Err(unexpected_end!())
    } else {
        Ok(keywords.iter().any(|&k| expect_keyword(tokens, k).unwrap()))
    }
}

pub(crate) fn expect_token<'parser>(
    tokens: &'parser [Token],
    checker: TokenChecker,
) -> Result<bool, Error> {
    if tokens.len() == 0 {
        Err(unexpected_end!())
    } else {
        Ok(checker(&tokens[0]))
    }
}

pub(crate) fn expect_one_of_tokens<'parser>(
    tokens: &'parser [Token],
    checkers: &'parser [TokenChecker],
) -> Result<bool, Error> {
    if tokens.len() == 0 {
        Err(unexpected_end!())
    } else {
        Ok(checkers.iter().any(|&c| expect_token(tokens, c).unwrap()))
    }
}

pub(crate) fn maybe_parse_tags<'parser>(
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
