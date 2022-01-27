//! Top level handling of definitions

use crate::error::Error;
use crate::tokenizer::Token;

use crate::parser::{
    asn::structs::{
        defs::{
            Asn1AssignmentKind, Asn1Definition, Asn1ObjectAssignment, Asn1ObjectClassAssignment,
            Asn1ObjectSetAssignment, Asn1TypeAssignment, Asn1ValueAssignment, DefinitionParam,
            DefinitionParams, DummyReferenceKind, GovernerKind, ParamDummyReference, ParamGoverner,
        },
        types::{
            ioc::{Asn1Object, Asn1ObjectSet, Asn1ObjectValue},
            ActualParam, Asn1Type,
        },
    },
    utils::{
        expect_keyword, expect_one_of_tokens, expect_token, expect_tokens, parse_set_ish_value,
    },
};

use super::types::{
    ioc::{parse_class, parse_object_from_class, parse_object_set, parse_object_set_from_class},
    parse_type,
};
use super::values::parse_value;

impl Asn1Definition {
    pub fn is_object_or_object_set(&self) -> bool {
        matches!(
            self.kind,
            Asn1AssignmentKind::ObjectSet(..) | Asn1AssignmentKind::Object(..)
        )
    }

    pub fn get_object_class(&self) -> Option<String> {
        match self.kind {
            Asn1AssignmentKind::ObjectSet(ref s) => Some(s.set.class.clone()),
            Asn1AssignmentKind::Object(ref o) => Some(o.object.class.clone()),
            _ => None,
        }
    }

    pub fn resolve_object_class(&mut self, class: &Asn1Definition) -> Result<(), Error> {
        if let Asn1AssignmentKind::Class(ref c) = class.kind {
            match self.kind {
                Asn1AssignmentKind::Object(ref mut o) => {
                    if let Asn1ObjectValue::Input(s) = &o.object.value {
                        let parsed = parse_object_from_class(&s, &c.classref)?;
                        o.object.value = parsed;
                    }
                }
                Asn1AssignmentKind::ObjectSet(ref mut s) => {
                    parse_object_set_from_class(&mut s.set, &c.classref)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn apply_params(&self, actual_params: &[ActualParam]) -> Result<Asn1Type, Error> {
        if self.params.is_none() {
            return Err(parse_error!("apply_params: No Params for the definition!"));
        }
        let mut params = self.params.as_ref().unwrap().clone();

        if params.ordered.len() != actual_params.len() {
            return Err(parse_error!(
                "Actual Params and Definition Params Lengths mismatch!"
            ));
        }

        for (idx, actual) in actual_params.iter().enumerate() {
            let replace = actual.param_string();
            let source = &params.ordered[idx].dummyref.name;

            for token in params.type_tokens.iter_mut() {
                if &token.text == source {
                    token.text = replace.clone()
                }
            }
        }
        let (ty, _) = parse_type(&params.type_tokens)?;
        Ok(ty)
    }
}
// Parse a definition into an `Assignment` type.
//
// This function is called by the Module parser to parse each definition. A definition will be one
// of the following assignment kinds - `ValueAssignment`, `TypeAssignment`, `ObjectClassAssignment`
// or `ObjectAssigmnet` or `ObjectSetAssignment`
//
// `ParameterizedAssignment` is supported for only `TypeAssignment`.
pub(super) fn parse_definition(tokens: &[Token]) -> Result<(Asn1Definition, usize), Error> {
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
        eprintln!("token: {:#?}", tokens[consumed]);
        Err(parse_error!("Not Implemented!"))
    }
}

// Parse `ValueAssignment` and `ObjectAssignment`
//
// All the above assignments start with a lowe-case letter and will have to be parsed into their
// respective 'values'. Returns the corresponding variant of the `Asn1Definition` and  the number
// of tokens consumed or error.
fn parse_valueish_definition(tokens: &[Token]) -> Result<(Asn1Definition, usize), Error> {
    if let Ok(x) = parse_object_assignment(tokens) {
        return Ok(x);
    }

    if let Ok(x) = parse_value_assignment(tokens) {
        return Ok(x);
    }

    Err(parse_error!(
        "Failed to parse a definition at Token: {:#?}",
        tokens[0]
    ))
}

// Parse object Assginemtnt
//
// object CLASS ::= { ... -- Object Defined Syntax -- }
fn parse_object_assignment(tokens: &[Token]) -> Result<(Asn1Definition, usize), Error> {
    let mut consumed = 0;

    if !expect_tokens(
        &tokens[consumed..],
        &[
            &[Token::is_object_reference],
            &[Token::is_object_class_reference],
        ],
    )? {
        return Err(unexpected_token!(
            "'object', 'CLASS' Reference",
            tokens[consumed]
        ));
    }
    let id = tokens[consumed].text.clone();
    consumed += 1;

    let class = tokens[consumed].text.clone();
    consumed += 1;

    if !expect_token(&tokens[consumed..], Token::is_assignment)? {
        return Err(unexpected_token!("::=", tokens[consumed]));
    }
    consumed += 1;

    let (value, value_consumed) = parse_set_ish_value(&tokens[consumed..])?;
    consumed += value_consumed;

    Ok((
        Asn1Definition {
            kind: Asn1AssignmentKind::Object(Asn1ObjectAssignment {
                id,
                object: Asn1Object {
                    class,
                    value: Asn1ObjectValue::Input(value),
                },
            }),
            params: None,
            resolved: false,
        },
        consumed,
    ))
}

// Parse object Assginemtnt
//
// value Type ::= ValueDefinition -- Value Definition can be anything for now.
fn parse_value_assignment(tokens: &[Token]) -> Result<(Asn1Definition, usize), Error> {
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
            resolved: false,
        },
        consumed,
    ))
}

// Parse A `TypeAssignment`, a `ObjectClassAssignement or `ObjectSetAssignment`
//
// All the above assignments start with a lowe-case letter and will have to be parsed into their
// respective 'values'. Returns the corresponding variant of the `Asn1Definition` and  the number
// of tokens consumed or error.
fn parse_typeish_definition(tokens: &[Token]) -> Result<(Asn1Definition, usize), Error> {
    // Try to parse a type_definition
    if let Ok(x) = parse_type_assignment(tokens) {
        return Ok(x);
    }

    if let Ok(x) = parse_class_assignment(tokens) {
        return Ok(x);
    }

    if let Ok(x) = parse_object_set_assignment(tokens) {
        return Ok(x);
    }

    Err(parse_error!(
        "Failed to parse a definition at Token: {:#?}",
        tokens[0]
    ))
}

// Parse a Type Assignment
//
// Identifier [{Params}] :== {Type|Refere} [(Constraints)]
fn parse_type_assignment(tokens: &[Token]) -> Result<(Asn1Definition, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_type_reference)? {
        return Err(unexpected_token!("Type Reference", tokens[consumed]));
    }
    let id = tokens[consumed].text.clone();
    consumed += 1;

    // Parse Optional Params
    let (params, params_consumed) = match parse_params(&tokens[consumed..]) {
        Ok(result) => (Some(result.0), result.1),
        Err(_) => (None, 0),
    };
    consumed += params_consumed;

    if !expect_token(&tokens[consumed..], Token::is_assignment)? {
        return Err(unexpected_token!("::=", tokens[consumed]));
    }
    consumed += 1;

    let start = consumed;

    let (typeref, typeref_consumed) = parse_type(&tokens[consumed..])?;
    consumed += typeref_consumed;

    let params = match params {
        Some(mut pars) => {
            pars.type_tokens = tokens[start..start + typeref_consumed].to_vec();
            Some(pars)
        }
        None => None,
    };

    Ok((
        Asn1Definition {
            kind: Asn1AssignmentKind::Type(Asn1TypeAssignment { id, typeref }),
            params,
            resolved: false,
        },
        consumed,
    ))
}

// Parse An Object CLASS Assignment
//
// CLASS-NAME :== CLASS { .... -- CLASS DEFINITION -- }
// Parameterized Class assignment not supported.
fn parse_class_assignment(tokens: &[Token]) -> Result<(Asn1Definition, usize), Error> {
    let mut consumed = 0;
    if !expect_token(&tokens[consumed..], Token::is_object_class_reference)? {
        return Err(unexpected_token!("CLASS Reference", tokens[consumed]));
    }
    let id = tokens[consumed].text.clone();
    consumed += 1;

    if !expect_token(&tokens[consumed..], Token::is_assignment)? {
        return Err(unexpected_token!("::=", tokens[consumed]));
    }
    consumed += 1;

    if !expect_keyword(&tokens[consumed..], "CLASS")? {
        return Err(unexpected_token!("'CLASS'", tokens[consumed]));
    }

    let (classref, classref_consumed) = parse_class(&tokens[consumed..])?;
    consumed += classref_consumed;

    Ok((
        Asn1Definition {
            kind: Asn1AssignmentKind::Class(Asn1ObjectClassAssignment { id, classref }),
            params: None,
            resolved: false,
        },
        consumed,
    ))
}

// Parse an Object Set Assignment
//
// ObjectSetName CLASS ::= { Objects } -- Where Objects can be an Object/ObjectSet/Reference
fn parse_object_set_assignment(tokens: &[Token]) -> Result<(Asn1Definition, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_type_reference)? {
        return Err(unexpected_token!("'Type Reference'", tokens[consumed]));
    }
    let id = tokens[consumed].text.clone();
    consumed += 1;

    if !expect_token(&tokens[consumed..], Token::is_object_class_reference)? {
        return Err(unexpected_token!("'CLASS Reference'", tokens[consumed]));
    }
    let class = tokens[consumed].text.clone();
    consumed += 1;

    if !expect_token(&tokens[consumed..], Token::is_assignment)? {
        return Err(unexpected_token!("'::='", tokens[consumed]));
    }
    consumed += 1;

    let (objects, objects_consumed) = parse_object_set(&tokens[consumed..])?;
    consumed += objects_consumed;

    Ok((
        Asn1Definition {
            kind: Asn1AssignmentKind::ObjectSet(Asn1ObjectSetAssignment {
                id,
                set: Asn1ObjectSet { class, objects },
            }),
            params: None,
            resolved: false,
        },
        consumed,
    ))
}

fn parse_params(tokens: &[Token]) -> Result<(DefinitionParams, usize), Error> {
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
            } else if governer.kind == GovernerKind::Class {
                ParamDummyReference {
                    name: r.text.clone(),
                    kind: DummyReferenceKind::ObjectSet,
                }
            } else {
                ParamDummyReference {
                    name: r.text.clone(),
                    kind: DummyReferenceKind::ValueSet,
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

    let params = DefinitionParams {
        ordered: params,
        type_tokens: vec![],
    };

    Ok((params, consumed))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn parse_definition_tests() {
        struct ParseDefinitionTestCase<'tc> {
            input: &'tc str,
            success: bool,
        }

        // This
        let test_cases = vec![
            ParseDefinitionTestCase {
                input: r#"
PDCP-Capability-r4-ext ::=  SEQUENCE {
    supportForRfc3095   CHOICE {
        notSupported    NULL,
        supported SEQUENCE {
            maxROHC-ContextSessions     MaxROHC-ContextSessions-r4  DEFAULT s16,
            reverseCompressionDepth     INTEGER (0..65535)          DEFAULT 0
        }
    }
}
"#,
                success: true,
            },
            ParseDefinitionTestCase {
                input: r#"
S1AP-PROTOCOL-IES ::= CLASS {
    &id             ProtocolIE-ID   UNIQUE,
    &criticality    Criticality,
    &Value,
    &presence       Presence
}
WITH SYNTAX {
    ID          &id
    CRITICALITY &criticality
    TYPE        &Value
    PRESENCE    &presence
}
"#,
                success: true,
            },
            ParseDefinitionTestCase {
                input: r#"
UE-RadioAccessCapability-LaterNonCriticalExtensions ::= SEQUENCE {
    v9c0NonCriticalExtensions   SEQUENCE {
        ue-RadioAccessCapability-v9c0ext    UE-RadioAccessCapability-v9c0ext-IEs,
        vaa0NonCriticalExtensions           SEQUENCE    {
            ue-RadioAccessCapability-vaa0ext    UE-RadioAccessCapability-vaa0ext-IEs,
            vb50NonCriticalExtensions           SEQUENCE {
                ue-RadioAccessCapability-vb50ext    UE-RadioAccessCapability-vb50ext-IEs,
                vb70NonCriticalExtensions   SEQUENCE {
                    ue-RadioAccessCapability-vb70ext    UE-RadioAccessCapability-vb70ext-IEs,
                    nonCriticalExtensions  SEQUENCE {}  OPTIONAL
                }   OPTIONAL
            }   OPTIONAL
        }   OPTIONAL
    }
}
"#,
                success: true,
            },
        ];

        for tc in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(tc.input));
            let tokens = tokenize(reader);
            assert!(tokens.is_ok());
            let tokens = tokens.unwrap();

            let def = parse_definition(&tokens);
            assert_eq!(
                def.is_ok(),
                tc.success,
                "{}:{}",
                tc.input,
                if tc.success {
                    format!("{:#?}", def.err())
                } else {
                    format!("{:#?}", def.ok())
                }
            );
        }

        assert!(true);
    }
}
