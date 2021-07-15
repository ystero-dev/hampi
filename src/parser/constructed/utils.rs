//! Utility functions for Parsing Constructed types

use crate::error::Error;
use crate::parser::{types::parse_type, utils::expect_token};
use crate::tokenizer::Token;

use crate::structs::constructed::Component;

pub(crate) fn parse_component<'parser>(
    tokens: &'parser [Token],
) -> Result<(Component, usize), Error> {
    let mut consumed = 0;
    if !expect_token(&tokens[consumed..], Token::is_value_reference)? {
        return Err(unexpected_token!("'IDENTIFIER'", tokens[consumed]));
    }
    let id = tokens[consumed].text.clone();
    consumed += 1;

    let (ty, ty_consumed) = parse_type(&tokens[consumed..])?;
    consumed += ty_consumed;

    Ok((Component { id, ty }, consumed))
}
