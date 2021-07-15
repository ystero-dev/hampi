//! Top level handling of definitions

use crate::error::Error;
use crate::structs::defs::{
    Asn1Definition, Asn1ObjectClassAssignment, Asn1TypeAssignment, Asn1ValueAssignment,
};
use crate::tokenizer::Token;

use super::ioc::parse_class;
use super::types::parse_type;
use super::utils::{expect_keyword, expect_one_of_tokens, expect_token, parse_set_ish_value};
use super::values::parse_value;

pub(super) fn parse_definition<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Definition, usize), Error> {
    let consumed = 0;

    if expect_one_of_tokens(
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
    // Try to parse a type_definition
    match parse_type_definition(tokens) {
        Ok(x) => {
            return Ok(x);
        }
        Err(_) => {}
    }

    match parse_class_definition(tokens) {
        Ok(x) => {
            return Ok(x);
        }
        Err(_) => {}
    }

    Err(parse_error!(
        "Failed to parse a definition at Token: {:#?}",
        tokens[0]
    ))
}

fn parse_type_definition<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Definition, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_type_reference)? {
        return Err(unexpected_token!("Type Reference", tokens[consumed]));
    }
    let id = tokens[consumed].text.clone();
    consumed += 1;

    if expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        eprintln!("1");
        let (_params, params_consumed) = parse_set_ish_value(&tokens[consumed..])?;
        consumed += params_consumed;
    }

    if !expect_token(&tokens[consumed..], Token::is_assignment)? {
        return Err(unexpected_token!("::=", tokens[consumed]));
    }
    consumed += 1;

    let (typeref, typeref_consumed) = parse_type(&tokens[consumed..])?;
    consumed += typeref_consumed;

    Ok((
        Asn1Definition::Type(Asn1TypeAssignment { id, typeref }),
        consumed,
    ))
}

fn parse_class_definition<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Definition, usize), Error> {
    let mut consumed = 0;
    if !expect_token(&tokens[consumed..], Token::is_object_class_reference)? {
        return Err(unexpected_token!("Type Reference", tokens[consumed]));
    }
    let id = tokens[consumed].text.clone();
    consumed += 1;

    if !expect_token(&tokens[consumed..], Token::is_assignment)? {
        return Err(unexpected_token!("::=", tokens[consumed]));
    }
    consumed += 1;

    if !expect_keyword(&tokens[consumed..], "CLASS")? {
        return Err(unexpected_token!("::=", tokens[consumed]));
    }

    let (classref, classref_consumed) = parse_class(&tokens[consumed..])?;
    consumed += classref_consumed;

    Ok((
        Asn1Definition::Class(Asn1ObjectClassAssignment { id, classref }),
        consumed,
    ))
}

// TODO: Add Test cases
