//! Top level handling of definitions

use crate::error::Error;
use crate::structs::defs::{Asn1Definition, Asn1ValueAssignment};
use crate::tokenizer::Token;

use super::types::parse_type;
use super::utils::{expect_token, expect_token_one_of};
use super::values::parse_value;

pub(crate) fn parse_definition<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Definition, usize), Error> {
    let consumed = 0;

    if expect_token_one_of(
        &tokens[consumed..],
        &[Token::is_value_reference, Token::is_type_reference],
    )? {
        let next = &tokens[consumed];
        if next.is_value_reference() {
            parse_valueish_definition(&tokens[consumed..])
        } else {
            parse_typeish_definition(&tokens[consumed..])
        }
    } else {
        eprintln!("Token: {:#?})", tokens[consumed]);
        Err(parse_error!("Not Implemented!"))
    }
}

// Parse `ValueAssignment`, `ObjectAssignment` and `ParameterizedValueAssignment` etc.
//
// All the above assignments start with a lowe-case letter and will have to be parsed into their
// respective 'values'. Returns the corresponding variant of the `Asn1Definition` and  the number
// of tokens consumed or error.
fn parse_valueish_definition<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Definition, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_value_reference)? {
        return Err(unexpected_token!("Value Reference", tokens[consumed]));
    }
    let id = tokens[consumed].text.clone();
    consumed += 1;

    let (typeref, typeref_consumed) = parse_type(&tokens[consumed..])?;
    consumed += typeref_consumed;

    if !expect_token(&tokens[consumed..], Token::is_assignment)? {
        return Err(unexpected_token!("::=", tokens[consumed]));
    }
    consumed += 1;

    let (value, value_consumed) = parse_value(&tokens[consumed..])?;
    consumed += value_consumed;

    Ok((
        Asn1Definition::Value(Asn1ValueAssignment { id, typeref, value }),
        consumed,
    ))
}

fn parse_typeish_definition<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Definition, usize), Error> {
    eprintln!("Token: {:#?}", tokens[0]);
    Err(parse_error!("Typeish Assignment Parsing Not Implemented"))
}

// TODO: Add Test cases
