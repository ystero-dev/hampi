use crate::error::Error;

use crate::parser::asn::structs::types::{Asn1BuiltinType, Asn1Type, Asn1TypeKind};

use crate::resolver::{
    asn::structs::types::base::{
        Asn1ResolvedBitString, Asn1ResolvedBoolean, Asn1ResolvedCharacterString,
        Asn1ResolvedEnumerated, Asn1ResolvedInteger, Asn1ResolvedObjectIdentifier,
        Asn1ResolvedOctetString, ResolvedBaseType,
    },
    Resolver,
};

pub(crate) fn resolve_base_type(
    ty: &Asn1Type,
    _resolver: &Resolver,
) -> Result<ResolvedBaseType, Error> {
    if let Asn1TypeKind::Builtin(ref kind) = ty.kind {
        match kind {
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
            Asn1BuiltinType::ObjectIdentifier => {
                let resolved =
                    ResolvedBaseType::ObjectIdentifier(Asn1ResolvedObjectIdentifier::default());
                Ok(resolved)
            }
            _ => Err(resolve_error!(
                "parse_base_type: Not Implemented! {:#?}",
                ty
            )),
        }
    } else {
        Err(resolve_error!("Expected Base Type. Found '{:#?}'", ty))
    }
}
