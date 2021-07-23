use std::collections::HashMap;

use crate::error::Error;

use crate::structs::parser::types::{Asn1Type, Asn1TypeKind};
use crate::structs::resolver::defs::Asn1ResolvedDefinition;
use crate::structs::resolver::types::Asn1ResolvedType;

pub(crate) mod base;
pub(crate) mod constructed;

use base::resolve_base_type;
use constructed::resolve_constructed_type;

pub(crate) fn resolve_type(
    ty: &Asn1Type,
    table: &HashMap<String, Asn1ResolvedDefinition>,
) -> Result<Asn1ResolvedType, Error> {
    match ty.kind {
        Asn1TypeKind::Builtin(ref b) => Ok(Asn1ResolvedType::Base(resolve_base_type(b, table)?)),
        Asn1TypeKind::Constructed(ref c) => Ok(Asn1ResolvedType::Constructed(
            resolve_constructed_type(c, table)?,
        )),
        _ => Err(resolve_error!("{:#?}", ty.kind)),
    }
}
