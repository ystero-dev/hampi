//! Utility functions for the parser module

use crate::error::Error;
use crate::tokenizer::{types::TokenChecker, Token};

// Returns true if the first `token`'s text is same as the passed `keyword`
pub(super) fn expect_keyword<'parser>(
    tokens: &'parser [Token],
    keyword: &str,
) -> Result<bool, Error> {
    if tokens.len() == 0 {
        Err(unexpected_end!())
    } else {
        Ok(tokens[0].is_given_keyword(keyword))
    }
}

// Returns true if all the given keywords are matched.
pub(super) fn expect_keywords<'parser>(
    tokens: &'parser [Token],
    keywords: &[&str],
) -> Result<bool, Error> {
    if tokens.len() < keywords.len() {
        Err(unexpected_end!())
    } else {
        Ok(keywords
            .iter()
            .zip(tokens.iter())
            .all(|(k, t)| t.is_given_keyword(k)))
    }
}

// Returns true if the first `token`'s text is one of the passed `keywords`.
pub(super) fn expect_one_of_keywords<'parser>(
    tokens: &'parser [Token],
    keywords: &[&str],
) -> Result<bool, Error> {
    if tokens.len() == 0 {
        Err(unexpected_end!())
    } else {
        Ok(keywords.iter().any(|&k| expect_keyword(tokens, k).unwrap()))
    }
}

// Returns `checker(token)` for the first `token`.
pub(super) fn expect_token<'parser>(
    tokens: &'parser [Token],
    checker: TokenChecker,
) -> Result<bool, Error> {
    if tokens.len() == 0 {
        Err(unexpected_end!())
    } else {
        Ok(checker(&tokens[0]))
    }
}

// Returns if any of the `checker(token)` returns true (short-circuiting)
pub(super) fn expect_one_of_tokens<'parser>(
    tokens: &'parser [Token],
    checkers: &'parser [TokenChecker],
) -> Result<bool, Error> {
    if tokens.len() == 0 {
        Err(unexpected_end!())
    } else {
        Ok(checkers.iter().any(|&c| expect_token(tokens, c).unwrap()))
    }
}

// Returns success if 'all' the 'tokens' return 'true' for `checker(token)`.
pub(super) fn expect_tokens<'parser>(
    tokens: &'parser [Token],
    checkers: &'parser [&'parser [TokenChecker]],
) -> Result<bool, Error> {
    if tokens.len() < checkers.len() {
        Err(unexpected_end!())
    } else {
        Ok(checkers
            .iter()
            .zip(tokens.iter())
            .all(|(inner_tokens, t)| inner_tokens.iter().any(|c| c(t))))
    }
}

// Consumes every thing between balanced "{" and "}" or "(" and ")".
//
// Unbalanced paranthesis is an error.
pub(super) fn parse_set_ish_value<'parser>(
    tokens: &'parser [Token],
) -> Result<(String, usize), Error> {
    let (begin_token, end_token): (TokenChecker, TokenChecker) =
        if expect_token(tokens, Token::is_curly_begin)? {
            (Token::is_curly_begin, Token::is_curly_end)
        } else if expect_token(tokens, Token::is_round_begin)? {
            (Token::is_round_begin, Token::is_round_end)
        } else {
            return Err(unexpected_token!("'(' or '}'", tokens[0]));
        };

    let mut level = 0;
    let mut consumed = 0;

    loop {
        if expect_token(&tokens[consumed..], begin_token)? {
            level += 1;
        }
        consumed += 1;
        if expect_token(&tokens[consumed..], end_token)? {
            level -= 1;
            consumed += 1;
            if level == 0 {
                return Ok((Token::concat(&tokens[0..consumed]), consumed));
            }
        }
    }
}

// TODO: Test Cases
