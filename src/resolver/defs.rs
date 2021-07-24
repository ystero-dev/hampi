//! 'defs' Resolver
use std::collections::HashMap;

use crate::error::Error;

use crate::structs::parser::defs::{
    Asn1AssignmentKind, Asn1Definition, Asn1TypeAssignment, Asn1ValueAssignment,
};
use crate::structs::resolver::defs::Asn1ResolvedDefinition;

use super::types::resolve_type;
use super::values::resolve_value;

// Resolve a given Parsed Definition to a Resolved Definition
//
// This function uses definitions already resolved in table to resolve the given definition.
pub(crate) fn resolve_definition(
    definition: &Asn1Definition,
    table: &HashMap<String, Asn1ResolvedDefinition>,
) -> Result<Asn1ResolvedDefinition, Error> {
    match definition.kind {
        Asn1AssignmentKind::Value(ref v) => resolve_value_definition(v, table),
        Asn1AssignmentKind::Type(ref t) => resolve_type_definition(t, table),
        _ => Err(resolve_error!(
            "asn_resolve_def: Not Implemented! {:#?}",
            definition
        )),
    }
}

fn resolve_type_definition(
    def: &Asn1TypeAssignment,
    table: &HashMap<String, Asn1ResolvedDefinition>,
) -> Result<Asn1ResolvedDefinition, Error> {
    let typeref = resolve_type(&def.typeref, table)?;
    Ok(Asn1ResolvedDefinition::Type(typeref))
}

pub(crate) fn resolve_value_definition(
    value: &Asn1ValueAssignment,
    table: &HashMap<String, Asn1ResolvedDefinition>,
) -> Result<Asn1ResolvedDefinition, Error> {
    let typeref = resolve_type(&value.typeref, table)?;
    let value = resolve_value(&value.value, typeref)?;
    Ok(Asn1ResolvedDefinition::Value(value))
}
