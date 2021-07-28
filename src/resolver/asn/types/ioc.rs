//! Handling of Information Object Classes, ObjectSets, Objects etc
use std::collections::HashMap;

use crate::error::Error;
use crate::tokenizer::{tokenize, Token};

use crate::parser::asn::{
    structs::{
        defs::Asn1AssignmentKind,
        types::ioc::{Asn1Object, Asn1ObjectSet, ObjectClassFieldSpec, ObjectSetElement},
    },
    types::parse_type,
    values::parse_value,
};

use crate::resolver::{
    asn::{
        structs::{
            defs::Asn1ResolvedDefinition,
            types::ioc::{
                Asn1ResolvedObject, Asn1ResolvedObjectSet, ResolvedFieldSpec, ResolvedObjectSet,
                ResolvedObjectSetElement,
            },
        },
        types::resolve_type,
        values::resolve_value,
    },
    Resolver,
};

pub(crate) fn resolve_object_set(
    objectset: &Asn1ObjectSet,
    resolver: &Resolver,
) -> Result<Asn1ResolvedObjectSet, Error> {
    let class = resolver.classes.get(&objectset.class);
    if class.is_none() {
        Err(resolve_error!(
            "Class '{}' definition not found to resolve object set!",
            objectset.class
        ))
    } else {
        let mut elements = vec![];
        for object in &objectset.objects.root_elements {
            let element = match object {
                ObjectSetElement::ObjectSetReference(ref r)
                | ObjectSetElement::ObjectReference(ref r) => {
                    let resolved = resolver.resolved_defs.get(r);
                    if resolved.is_none() {
                        return Err(resolve_error!(
                            "Unable to find the Referencing '{}' Object Set while resolving {:#?}",
                            r,
                            objectset
                        ));
                    } else {
                        let resolved = resolved.unwrap();
                        let element = match resolved {
                            Asn1ResolvedDefinition::ObjectSet(..) => {
                                ResolvedObjectSetElement::ObjectSetReference(r.clone())
                            }
                            Asn1ResolvedDefinition::Object(..) => {
                                ResolvedObjectSetElement::ObjectReference(r.clone())
                            }
                            _ => {
                                return Err(resolve_error!(
                                    "Matching definition found for reference '{}' but is a '{:#?}' and not an ObjectSet or an Object!",
                                    r,
                                    resolved,
                                ));
                            }
                        };
                        element
                    }
                }
                ObjectSetElement::Object(ref v) => ResolvedObjectSetElement::Object(
                    resolve_object_value_for_class(v, &objectset.class, resolver)?,
                ),
            };
            elements.push(element);
        }
        Ok(Asn1ResolvedObjectSet {
            objects: ResolvedObjectSet { elements },
        })
    }
}

pub(crate) fn resolve_object(
    object: &Asn1Object,
    resolver: &Resolver,
) -> Result<Asn1ResolvedObject, Error> {
    resolve_object_value_for_class(&object.value, &object.class, resolver)
}

fn resolve_object_value_for_class(
    value: &String,
    class: &String,
    resolver: &Resolver,
) -> Result<Asn1ResolvedObject, Error> {
    let classdef = resolver.classes.get(class);
    if classdef.is_none() {
        Err(resolve_error!(
            "Class '{}' definition not found to resolve object!",
            class
        ))
    } else {
        let classdef = classdef.unwrap();
        if let Asn1AssignmentKind::Class(ref c) = classdef.kind {
            let mut class = c.classref.clone();
            let reader = std::io::BufReader::new(std::io::Cursor::new(value));
            let tokens = tokenize(reader)?;
            let mut consumed = 0;
            let object_tokens = &tokens[1..tokens.len() - 1];
            let word_tokens = &mut object_tokens.split(|t| !t.is_with_syntax_word());
            let mut fields = HashMap::new();
            loop {
                let words = word_tokens.next();
                if words.is_none() {
                    break;
                } else {
                    let words = words.unwrap();
                    if words.len() > 0 {
                        consumed += words.len();
                        let words = words
                            .iter()
                            .map(|t| t.text.clone())
                            .collect::<Vec<String>>()
                            .join(" ");
                        // FIXME: call a 'Resolve_Spec' function
                        for (field, spec) in class.fields.iter_mut() {
                            if !spec.resolved() {
                                let (resolved, resolved_consumed) = match spec {
                                    ObjectClassFieldSpec::Type { .. } => {
                                        match resolve_type_and_field_spec(
                                            &words,
                                            spec,
                                            resolver,
                                            &object_tokens[consumed..],
                                        ) {
                                            Ok(result) => result,
                                            Err(_) => (None, 0),
                                        }
                                    }
                                    ObjectClassFieldSpec::FixedTypeValue { .. } => {
                                        // FIXME : Assumes a simple value, but can be a constructed
                                        // one, for now ignore.
                                        match resolve_fixed_type_value_field_spec(
                                            &words,
                                            spec,
                                            resolver,
                                            &object_tokens[consumed..],
                                        ) {
                                            Ok(result) => result,
                                            Err(_) => (None, 0),
                                        }
                                    }
                                };
                                if resolved.is_some() {
                                    consumed += resolved_consumed;
                                    fields.insert(field.clone(), resolved.unwrap());
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Ok(Asn1ResolvedObject { fields })
        } else {
            Err(resolve_error!(
                "Definition for '{}' found, but is not a Class Definition!",
                class
            ))
        }
    }
}

fn resolve_type_and_field_spec(
    words: &String,
    spec: &mut ObjectClassFieldSpec,
    resolver: &Resolver,
    tokens: &[Token],
) -> Result<(Option<ResolvedFieldSpec>, usize), Error> {
    // First get a Type for this

    if let ObjectClassFieldSpec::Type {
        with_syntax,
        resolved,
        ..
    } = spec
    {
        if with_syntax.is_some() {
            if with_syntax.as_ref().unwrap() == words {
                let (parsed_type, consumed) = parse_type(tokens)?;
                let ty = resolve_type(&parsed_type, resolver)?;
                *resolved = true;
                Ok((Some(ResolvedFieldSpec::Type { ty }), consumed))
            } else {
                Ok((None, 0))
            }
        } else {
            Ok((None, 0))
        }
    } else {
        Err(resolve_error!("Invalid Variant"))
    }
}

fn resolve_fixed_type_value_field_spec(
    words: &String,
    spec: &mut ObjectClassFieldSpec,
    resolver: &Resolver,
    tokens: &[Token],
) -> Result<(Option<ResolvedFieldSpec>, usize), Error> {
    let (value, value_consumed) = parse_value(tokens)?;

    if let ObjectClassFieldSpec::FixedTypeValue {
        with_syntax,
        resolved,
        field_type,
        ..
    } = spec
    {
        if with_syntax.is_some() {
            if with_syntax.as_ref().unwrap() == words {
                *resolved = true;
                let typeref = resolve_type(field_type, resolver)?;
                let value = resolve_value(&value, typeref, resolver)?;
                Ok((
                    Some(ResolvedFieldSpec::FixedTypeValue { value }),
                    value_consumed,
                ))
            } else {
                Ok((None, 0))
            }
        } else {
            Ok((None, 0))
        }
    } else {
        Err(resolve_error!("Invalid Variant"))
    }
}
