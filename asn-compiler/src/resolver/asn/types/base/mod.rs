//! Handling of Resolution of 'base' types.

mod integer;

mod enumerated;

mod bitstring;

mod octetstring;

mod charstring;

use crate::error::Error;

use crate::parser::asn::structs::types::{Asn1BuiltinType, Asn1Type, Asn1TypeKind};

use crate::resolver::{
    asn::structs::types::base::{
        Asn1ResolvedBitString, Asn1ResolvedBoolean, Asn1ResolvedCharacterString,
        Asn1ResolvedEnumerated, Asn1ResolvedInteger, Asn1ResolvedNull,
        Asn1ResolvedObjectIdentifier, Asn1ResolvedOctetString, Asn1ResolvedReal, ResolvedBaseType,
    },
    Resolver,
};

pub(crate) fn resolve_base_type(
    ty: &Asn1Type,
    resolver: &mut Resolver,
) -> Result<ResolvedBaseType, Error> {
    if let Asn1TypeKind::Builtin(ref kind) = ty.kind {
        match kind {
            Asn1BuiltinType::Integer(ref i) => Ok(ResolvedBaseType::Integer(
                Asn1ResolvedInteger::resolve_integer(ty, i, resolver)?,
            )),
            Asn1BuiltinType::Enumerated(ref e) => Ok(ResolvedBaseType::Enum(
                Asn1ResolvedEnumerated::resolve_enumerated(ty, e, resolver)?,
            )),
            Asn1BuiltinType::BitString(ref b) => Ok(ResolvedBaseType::BitString(
                Asn1ResolvedBitString::resolve_bit_string(ty, b, resolver)?,
            )),
            Asn1BuiltinType::Boolean => Ok(ResolvedBaseType::Boolean(Asn1ResolvedBoolean)),
            Asn1BuiltinType::OctetString => Ok(ResolvedBaseType::OctetString(
                Asn1ResolvedOctetString::resolve_octet_string(ty, resolver)?,
            )),
            Asn1BuiltinType::CharacterString { .. } => Ok(ResolvedBaseType::CharacterString(
                Asn1ResolvedCharacterString::resolve_character_string(ty, resolver)?,
            )),
            Asn1BuiltinType::ObjectIdentifier => Ok(ResolvedBaseType::ObjectIdentifier(
                Asn1ResolvedObjectIdentifier,
            )),
            Asn1BuiltinType::Null => Ok(ResolvedBaseType::Null(Asn1ResolvedNull)),
            Asn1BuiltinType::Real => Ok(ResolvedBaseType::Real(Asn1ResolvedReal)),
            _ => Err(resolve_error!(
                "parse_base_type: Not Implemented! {:#?}",
                ty
            )),
        }
    } else {
        Err(resolve_error!("Expected Base Type. Found '{:#?}'", ty))
    }
}
