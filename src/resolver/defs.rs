//! 'defs' Resolver

use crate::error::Error;

use crate::structs::parser::defs::{
    Asn1AssignmentKind, Asn1Definition, Asn1ObjectAssignment, Asn1ObjectSetAssignment,
    Asn1TypeAssignment, Asn1ValueAssignment,
};
use crate::structs::resolver::{defs::Asn1ResolvedDefinition, Resolver};

use super::types::ioc::{resolve_object, resolve_object_set};
use super::types::resolve_type;
use super::values::resolve_value;

// Resolve a given Parsed Definition to a Resolved Definition
//
// This function uses definitions already resolved in table to resolve the given definition.
pub(crate) fn resolve_definition(
    definition: &Asn1Definition,
    resolver: &mut Resolver,
) -> Result<Asn1ResolvedDefinition, Error> {
    match definition.kind {
        Asn1AssignmentKind::Value(ref v) => resolve_value_definition(v, resolver),
        Asn1AssignmentKind::Type(ref t) => resolve_type_definition(t, resolver),
        Asn1AssignmentKind::ObjectSet(ref objset) => {
            resolve_object_set_definition(objset, resolver)
        }
        Asn1AssignmentKind::Object(ref object) => resolve_object_definition(object, resolver),
        _ => Err(resolve_error!(
            "asn_resolve_def: Not Implemented! {:#?}",
            definition
        )),
    }
}

pub(crate) fn resolve_type_definition(
    def: &Asn1TypeAssignment,
    resolver: &mut Resolver,
) -> Result<Asn1ResolvedDefinition, Error> {
    let typeref = resolve_type(&def.typeref, resolver)?;
    Ok(Asn1ResolvedDefinition::Type(typeref))
}

fn resolve_value_definition(
    value: &Asn1ValueAssignment,
    resolver: &mut Resolver,
) -> Result<Asn1ResolvedDefinition, Error> {
    let typeref = resolve_type(&value.typeref, resolver)?;
    let value = resolve_value(&value.value, typeref)?;
    Ok(Asn1ResolvedDefinition::Value(value))
}

fn resolve_object_set_definition(
    objectset: &Asn1ObjectSetAssignment,
    resolver: &mut Resolver,
) -> Result<Asn1ResolvedDefinition, Error> {
    let objectset = resolve_object_set(&objectset.set, resolver)?;
    Ok(Asn1ResolvedDefinition::ObjectSet(objectset))
}

fn resolve_object_definition(
    object: &Asn1ObjectAssignment,
    resolver: &mut Resolver,
) -> Result<Asn1ResolvedDefinition, Error> {
    let object = resolve_object(&object.object, resolver)?;
    Ok(Asn1ResolvedDefinition::Object(object))
}
