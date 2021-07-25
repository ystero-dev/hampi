//! 'defs' Resolver
use std::collections::HashMap;

use crate::error::Error;

use crate::structs::parser::defs::{
    Asn1AssignmentKind, Asn1Definition, Asn1ObjectAssignment, Asn1ObjectClassAssignment,
    Asn1ObjectSetAssignment, Asn1TypeAssignment, Asn1ValueAssignment,
};
use crate::structs::resolver::defs::Asn1ResolvedDefinition;

use super::types::ioc::{resolve_object, resolve_object_set};
use super::types::resolve_type;
use super::values::resolve_value;

// Resolve a given Parsed Definition to a Resolved Definition
//
// This function uses definitions already resolved in table to resolve the given definition.
pub(crate) fn resolve_definition(
    definition: &Asn1Definition,
    resolved_defs: &HashMap<String, Asn1ResolvedDefinition>,
    parameterized_defs: &HashMap<String, Asn1Definition>,
    object_classes: &HashMap<String, Asn1ObjectClassAssignment>,
) -> Result<Asn1ResolvedDefinition, Error> {
    match definition.kind {
        Asn1AssignmentKind::Value(ref v) => {
            resolve_value_definition(v, resolved_defs, parameterized_defs, object_classes)
        }
        Asn1AssignmentKind::Type(ref t) => {
            resolve_type_definition(t, resolved_defs, parameterized_defs, object_classes)
        }
        Asn1AssignmentKind::ObjectSet(ref objset) => {
            resolve_object_set_definition(objset, resolved_defs, parameterized_defs, object_classes)
        }
        Asn1AssignmentKind::Object(ref object) => {
            resolve_object_definition(object, resolved_defs, parameterized_defs, object_classes)
        }
        _ => Err(resolve_error!(
            "asn_resolve_def: Not Implemented! {:#?}",
            definition
        )),
    }
}

pub(crate) fn resolve_type_definition(
    def: &Asn1TypeAssignment,
    resolved_defs: &HashMap<String, Asn1ResolvedDefinition>,
    parameterized_defs: &HashMap<String, Asn1Definition>,
    object_classes: &HashMap<String, Asn1ObjectClassAssignment>,
) -> Result<Asn1ResolvedDefinition, Error> {
    let typeref = resolve_type(
        &def.typeref,
        resolved_defs,
        parameterized_defs,
        object_classes,
    )?;
    Ok(Asn1ResolvedDefinition::Type(typeref))
}

fn resolve_value_definition(
    value: &Asn1ValueAssignment,
    resolved_defs: &HashMap<String, Asn1ResolvedDefinition>,
    parameterized_defs: &HashMap<String, Asn1Definition>,
    object_classes: &HashMap<String, Asn1ObjectClassAssignment>,
) -> Result<Asn1ResolvedDefinition, Error> {
    let typeref = resolve_type(
        &value.typeref,
        resolved_defs,
        parameterized_defs,
        object_classes,
    )?;
    let value = resolve_value(&value.value, typeref)?;
    Ok(Asn1ResolvedDefinition::Value(value))
}

fn resolve_object_set_definition(
    objectset: &Asn1ObjectSetAssignment,
    resolved_defs: &HashMap<String, Asn1ResolvedDefinition>,
    parameterized_defs: &HashMap<String, Asn1Definition>,
    object_classes: &HashMap<String, Asn1ObjectClassAssignment>,
) -> Result<Asn1ResolvedDefinition, Error> {
    let objectset = resolve_object_set(
        &objectset.set,
        resolved_defs,
        parameterized_defs,
        object_classes,
    )?;
    Ok(Asn1ResolvedDefinition::ObjectSet(objectset))
}

fn resolve_object_definition(
    object: &Asn1ObjectAssignment,
    resolved_defs: &HashMap<String, Asn1ResolvedDefinition>,
    parameterized_defs: &HashMap<String, Asn1Definition>,
    object_classes: &HashMap<String, Asn1ObjectClassAssignment>,
) -> Result<Asn1ResolvedDefinition, Error> {
    let object = resolve_object(
        &object.object,
        resolved_defs,
        parameterized_defs,
        object_classes,
    )?;
    Ok(Asn1ResolvedDefinition::Object(object))
}
