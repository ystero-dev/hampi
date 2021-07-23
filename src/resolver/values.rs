//! Resolved 'values' implementation

use std::collections::HashMap;

use crate::error::Error;

use crate::structs::parser::defs::Asn1ValueAssignment;
use crate::structs::resolver::defs::Asn1ResolvedDefinition;

pub(crate) fn resolve_value_definition(
    value: &Asn1ValueAssignment,
    _table: &HashMap<String, Asn1ResolvedDefinition>,
) -> Result<Asn1ResolvedDefinition, Error> {
    match value.typeref.kind {
        _ => Err(resolve_error!("{:#?}", value.typeref.kind)),
    }
}
