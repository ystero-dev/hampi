use std::collections::HashMap;

use crate::error::Error;

use crate::structs::parser::defs::Asn1TypeAssignment;
use crate::structs::parser::types::Asn1TypeKind;
use crate::structs::resolver::defs::Asn1ResolvedDefinition;

pub(crate) mod base;
pub(crate) mod constructed;

use base::resolve_base_type;
use constructed::resolve_constructed_type;

pub(crate) fn resolve_type_definition(
    ty: &Asn1TypeAssignment,
    table: &HashMap<String, Asn1ResolvedDefinition>,
) -> Result<Asn1ResolvedDefinition, Error> {
    match ty.typeref.kind {
        Asn1TypeKind::Builtin(ref b) => resolve_base_type(b, table),
        Asn1TypeKind::Constructed(ref c) => resolve_constructed_type(c, table),
        _ => Err(resolve_error!("{:#?}", ty.typeref.kind)),
    }
}
