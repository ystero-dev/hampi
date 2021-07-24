use std::collections::HashMap;

use crate::error::Error;

use crate::structs::parser::types::Asn1BuiltinType;
use crate::structs::resolver::{
    defs::Asn1ResolvedDefinition,
    types::base::{
        Asn1ResolvedBitString, Asn1ResolvedBoolean, Asn1ResolvedCharacterString,
        Asn1ResolvedEnumerated, Asn1ResolvedInteger, Asn1ResolvedOctetString, ResolvedBaseType,
    },
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
        Asn1BuiltinType::BitString(ref _i) => {
            let resolved = ResolvedBaseType::BitString(Asn1ResolvedBitString::default());
            Ok(resolved)
        }
        Asn1BuiltinType::Boolean => {
            let resolved = ResolvedBaseType::Boolean(Asn1ResolvedBoolean::default());
            Ok(resolved)
        }
        Asn1BuiltinType::OctetString => {
            let resolved = ResolvedBaseType::OctetString(Asn1ResolvedOctetString::default());
            Ok(resolved)
        }
        Asn1BuiltinType::CharacterString => {
            let resolved =
                ResolvedBaseType::CharacterString(Asn1ResolvedCharacterString::default());
            Ok(resolved)
        }
        _ => Err(resolve_error!(
            "parse_base_type: Not Implemented! {:#?}",
            ty
        )),
    }
}
