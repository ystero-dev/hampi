//! Parsing of Information Object Class, Objects, Object Sets etc.
use std::collections::HashMap;

use crate::error::Error;
use crate::structs::ioc::*;
use crate::tokenizer::Token;

use super::types::parse_type;
use super::utils::{
    expect_keyword, expect_keywords, expect_one_of_keywords, expect_token, parse_set_ish_value,
};
use super::values::parse_value;

pub(crate) fn parse_class<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1ObjectClass, usize), Error> {
    let mut consumed = 0;

    if !expect_keyword(&tokens[consumed..], "CLASS")? {
        return Err(unexpected_token!("'CLASS'", tokens[consumed]));
    }
    consumed += 1;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{'", tokens[consumed]));
    }
    consumed += 1;

    let mut fields = HashMap::new();
    loop {
        // Any error in parsing a field spec is an error.
        let (field_spec, field_spec_consumed) = parse_field_spec(&tokens[consumed..])?;
        consumed += field_spec_consumed;

        fields.insert(field_spec.id(), field_spec);

        if expect_token(&tokens[consumed..], Token::is_comma)? {
            consumed += 1;
        } else {
            if expect_token(&tokens[consumed..], Token::is_curly_end)? {
                consumed += 1;
                break;
            } else {
                return Err(unexpected_token!("',' or '}'", tokens[consumed]));
            }
        }
    }

    let with_syntax_consumed = parse_with_syntax_for_fields(&tokens[consumed..], &mut fields)?;
    consumed += with_syntax_consumed;

    Ok((Asn1ObjectClass { fields }, consumed))
}

fn parse_field_spec<'parser>(
    tokens: &'parser [Token],
) -> Result<(ObjectClassFieldSpec, usize), Error> {
    if expect_token(tokens, Token::is_value_field_reference)? {
        parse_fixed_type_value_field_spec(tokens)
    } else if expect_token(tokens, Token::is_type_field_reference)? {
        parse_type_field_spec(tokens)
    } else {
        Err(parse_error!("Unsupported Field Spec in CLASS Definition"))
    }
}

fn parse_fixed_type_value_field_spec<'parser>(
    tokens: &'parser [Token],
) -> Result<(ObjectClassFieldSpec, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_value_field_reference)? {
        return Err(unexpected_token!("'VALUE FIELD REF'", tokens[consumed]));
    }

    let id = tokens[consumed].text.clone();
    consumed += 1;

    let (field_type, field_type_consumed) = parse_type(&tokens[consumed..])?;
    consumed += field_type_consumed;

    let unique = match expect_keyword(&tokens[consumed..], "UNIQUE") {
        Ok(u) => {
            if u {
                consumed += 1;
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };

    let mut optional = false;
    let mut default = None;
    if expect_one_of_keywords(&tokens[consumed..], &["OPTIONAL", "DEFAULT"])? {
        if expect_keyword(&tokens[consumed..], "OPTIONAL")? {
            optional = match expect_keyword(&tokens[consumed..], "OPTIONAL") {
                Ok(o) => {
                    if o {
                        consumed += 1;
                        true
                    } else {
                        false
                    }
                }
                Err(_) => false,
            };
        } else {
            default = match expect_keyword(&tokens[consumed..], "DEFAULT") {
                Ok(x) => {
                    if x {
                        consumed += 1;
                        let (value, value_consumed) = parse_value(&tokens[consumed..])?;
                        consumed += value_consumed;
                        Some(value)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            };
        }

        if default.is_some() && unique {
            return Err(parse_error!(
                "Both 'UNIQUE' and 'DEFAULT' cannot be specified together!"
            ));
        }
    }

    let with_syntax = None;

    Ok((
        ObjectClassFieldSpec::FixedTypeValue(FixedTypeValueFieldSpec {
            id,
            field_type,
            unique,
            optional,
            default,
            with_syntax,
        }),
        consumed,
    ))
}

fn parse_type_field_spec<'parser>(
    tokens: &'parser [Token],
) -> Result<(ObjectClassFieldSpec, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_type_field_reference)? {
        return Err(unexpected_token!("'TYPE FIELD REF'", tokens[consumed]));
    }

    let id = tokens[consumed].text.clone();
    consumed += 1;

    let mut optional = false;
    let mut default = None;
    if expect_one_of_keywords(&tokens[consumed..], &["OPTIONAL", "DEFAULT"])? {
        if expect_keyword(&tokens[consumed..], "OPTIONAL")? {
            optional = match expect_keyword(&tokens[consumed..], "OPTIONAL") {
                Ok(o) => {
                    if o {
                        consumed += 1;
                        true
                    } else {
                        false
                    }
                }
                Err(_) => false,
            };
        } else {
            default = match expect_keyword(&tokens[consumed..], "DEFAULT") {
                Ok(x) => {
                    if x {
                        consumed += 1;
                        let (default, default_consumed) = parse_type(&tokens[consumed..])?;
                        consumed += default_consumed;
                        Some(default)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            };
        }
    }

    let with_syntax = None;

    Ok((
        ObjectClassFieldSpec::Type(TypeFieldSpec {
            id,
            optional,
            default,
            with_syntax,
        }),
        consumed,
    ))
}

fn parse_with_syntax_for_fields<'parser>(
    tokens: &'parser [Token],
    _fields: &'parser mut HashMap<String, ObjectClassFieldSpec>,
) -> Result<usize, Error> {
    let mut consumed = 0;
    if !expect_keywords(&tokens[consumed..], &["WITH", "SYNTAX"])? {
        return Ok(consumed);
    }
    consumed += 2;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{'", tokens[consumed]));
    }

    // FIXME: Handle Syntax properly. For this we will need to splice the words inside with-syntax.
    // For now just 'consume' the tokens.
    let (_, value_consumed) = parse_set_ish_value(&tokens[consumed..])?;
    consumed += value_consumed;

    Ok(consumed)
}
