use std::collections::HashMap;

use crate::error::Error;

use crate::structs::parser::defs::{Asn1AssignmentKind, Asn1Definition, Asn1ObjectClassAssignment};
use crate::structs::parser::types::{Asn1Type, Asn1TypeKind, Asn1TypeReference};
use crate::structs::resolver::defs::Asn1ResolvedDefinition;
use crate::structs::resolver::types::Asn1ResolvedType;

pub(crate) mod base;
pub(crate) mod constructed;
pub(crate) mod ioc;

use base::resolve_base_type;
use constructed::resolve_constructed_type;

pub(crate) fn resolve_type(
    ty: &Asn1Type,
    resolved_defs: &HashMap<String, Asn1ResolvedDefinition>,
    parameterized_defs: &HashMap<String, Asn1Definition>,
    object_classes: &HashMap<String, Asn1ObjectClassAssignment>,
) -> Result<Asn1ResolvedType, Error> {
    match ty.kind {
        Asn1TypeKind::Builtin(ref b) => {
            Ok(Asn1ResolvedType::Base(resolve_base_type(b, resolved_defs)?))
        }
        Asn1TypeKind::Constructed(ref c) => Ok(Asn1ResolvedType::Constructed(
            resolve_constructed_type(c, resolved_defs)?,
        )),
        Asn1TypeKind::Reference(ref r) => {
            resolve_reference_type(r, resolved_defs, parameterized_defs, object_classes)
        }
    }
}

fn resolve_reference_type(
    reference: &Asn1TypeReference,
    resolved_defs: &HashMap<String, Asn1ResolvedDefinition>,
    parameterized_defs: &HashMap<String, Asn1Definition>,
    object_classes: &HashMap<String, Asn1ObjectClassAssignment>,
) -> Result<Asn1ResolvedType, Error> {
    match reference {
        Asn1TypeReference::Reference(ref r) => {
            let resolved = resolved_defs.get(r);
            if resolved.is_some() {
                let resolved = resolved.unwrap();
                match resolved {
                    Asn1ResolvedDefinition::Type(ref t) => Ok(t.clone()),
                    _ => Err(resolve_error!(
                        "Expected a Resolved Type, found {:#?}",
                        resolved
                    )),
                }
            } else {
                Err(resolve_error!(
                    "Referenced Type for '{}' Not resolved yet!",
                    r
                ))
            }
        }
        Asn1TypeReference::Parameterized(ref p) => {
            let def = parameterized_defs.get(&p.typeref);
            if def.is_some() {
                let def = def.unwrap();
                match def.kind {
                    // FIXME : This is not exactly 'Right' but for now we'd go ahead with it.
                    Asn1AssignmentKind::Type(ref t) => resolve_type(
                        &t.typeref,
                        resolved_defs,
                        parameterized_defs,
                        object_classes,
                    ),
                    _ => Err(resolve_error!(
                        "parameterized_type of {:#?} kind not supported.",
                        def.kind
                    )),
                }
            } else {
                Err(resolve_error!(
                    "Parameterized Type for '{:#?}' Not found!",
                    p
                ))
            }
        }
        _ => Err(resolve_error!(
            "resolve_reference_type: Not Supported: {:#?}",
            reference
        )),
    }
}
