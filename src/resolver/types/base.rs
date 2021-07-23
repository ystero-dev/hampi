use std::collections::HashMap;

use crate::error::Error;

use crate::structs::parser::types::Asn1BuiltinType;
use crate::structs::resolver::{
    defs::Asn1ResolvedDefinition,
    types::base::{Asn1ResolvedEnumerated, Asn1ResolvedInteger, ResolvedBaseType},
};

pub(crate) fn resolve_base_type(
    ty: &Asn1BuiltinType,
    _table: &HashMap<String, Asn1ResolvedDefinition>,
) -> Result<ResolvedBaseType, Error> {
    match ty {
        Asn1BuiltinType::Integer(ref _i) => {
            let resolved = ResolvedBaseType::Integer(Asn1ResolvedInteger::default());
            Ok(resolved)
        }
        Asn1BuiltinType::Enumerated(ref _i) => {
            let resolved = ResolvedBaseType::Enum(Asn1ResolvedEnumerated::default());
            Ok(resolved)
        }
        _ => Err(resolve_error!("Not Implemented!")),
    }
}
