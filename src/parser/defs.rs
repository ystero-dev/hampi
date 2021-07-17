//! Top level handling of definitions

use crate::error::Error;
use crate::structs::defs::{
    Asn1AssignmentKind, Asn1Definition, Asn1ObjectClassAssignment, Asn1TypeAssignment,
    Asn1ValueAssignment, DefinitionParam, DummyReferenceKind, GovernerKind, ParamDummyReference,
    ParamGoverner,
};
use crate::tokenizer::Token;

use super::ioc::parse_class;
use super::types::parse_type;
use super::utils::{expect_keyword, expect_one_of_tokens, expect_token, expect_tokens};
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
        Asn1Definition {
            kind: Asn1AssignmentKind::Value(Asn1ValueAssignment { id, typeref, value }),
            params: None,
        },
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

    let (params, params_consumed) = match parse_params(&tokens[consumed..]) {
        Ok(result) => (Some(result.0), result.1),
        Err(_) => (None, 0),
    };
    consumed += params_consumed;

    if !expect_token(&tokens[consumed..], Token::is_assignment)? {
        return Err(unexpected_token!("::=", tokens[consumed]));
    }
    consumed += 1;

    let (typeref, typeref_consumed) = parse_type(&tokens[consumed..])?;
    consumed += typeref_consumed;

    Ok((
        Asn1Definition {
            kind: Asn1AssignmentKind::Type(Asn1TypeAssignment { id, typeref }),
            params,
        },
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
        Asn1Definition {
            kind: Asn1AssignmentKind::Class(Asn1ObjectClassAssignment { id, classref }),
            params: None,
        },
        consumed,
    ))
}

fn parse_params<'parser>(tokens: &'parser [Token]) -> Result<(Vec<DefinitionParam>, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{'", tokens[consumed]));
    }
    consumed += 1;

    let mut params = vec![];
    loop {
        // Try to parse the Governer: DummyReference if fails, whatever remains is a Type or Class
        let (governer, dummyref) = if expect_tokens(
            &tokens[consumed..],
            &[
                &[
                    Token::is_type_reference,
                    Token::is_asn_builtin_type,
                    Token::is_object_class_reference,
                ],
                &[Token::is_colon],
                &[Token::is_value_reference, Token::is_type_reference],
            ],
        )? {
            let (g, r) = (&tokens[consumed], &tokens[consumed + 2]);
            let governer = if g.is_object_class_reference() {
                ParamGoverner {
                    name: g.text.clone(),
                    kind: GovernerKind::Class,
                }
            } else {
                ParamGoverner {
                    name: g.text.clone(),
                    kind: GovernerKind::Type,
                }
            };
            let dummyref = if r.is_value_reference() {
                if governer.kind == GovernerKind::Class {
                    ParamDummyReference {
                        name: r.text.clone(),
                        kind: DummyReferenceKind::Object,
                    }
                } else {
                    ParamDummyReference {
                        name: r.text.clone(),
                        kind: DummyReferenceKind::Value,
                    }
                }
            } else {
                if governer.kind == GovernerKind::Class {
                    ParamDummyReference {
                        name: r.text.clone(),
                        kind: DummyReferenceKind::ObjectSet,
                    }
                } else {
                    ParamDummyReference {
                        name: r.text.clone(),
                        kind: DummyReferenceKind::ValueSet,
                    }
                }
            };
            consumed += 3;
            (Some(governer), dummyref)
        } else if expect_one_of_tokens(
            &tokens[consumed..],
            &[Token::is_type_reference, Token::is_object_class_reference],
        )? {
            let r = &tokens[consumed];
            consumed += 1;
            if r.is_object_class_reference() {
                (
                    None,
                    ParamDummyReference {
                        name: r.text.clone(),
                        kind: DummyReferenceKind::Class,
                    },
                )
            } else {
                (
                    None,
                    ParamDummyReference {
                        name: r.text.clone(),
                        kind: DummyReferenceKind::Type,
                    },
                )
            }
        } else {
            return Err(parse_error!(
                "Error in Parsing params at Token: {:#?}",
                tokens[consumed]
            ));
        };
        let param = DefinitionParam { governer, dummyref };
        params.push(param);

        if expect_token(&tokens[consumed..], Token::is_comma)? {
            consumed += 1;
        }

        if expect_token(&tokens[consumed..], Token::is_curly_end)? {
            consumed += 1;
            break;
        }
    }

    eprintln!("params: {:#?}", params);

    Ok((params, consumed))
}
// TODO: Add Test cases
