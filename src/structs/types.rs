//! Structures related to ASN.1 Type
use std::collections::HashMap;

use lazy_static::lazy_static;

use super::base::{Asn1TypeEnumerated, Asn1TypeInteger};
use super::constraints::Asn1Constraint;

lazy_static! {
    pub(crate) static ref ASN_BUILTIN_TYPE_KINDS: HashMap<&'static str, Asn1TypeKind> = {
        let mut m = HashMap::new();
        m.insert("BOOLEAN", Asn1TypeKind::Builtin(Asn1BuiltinType::Boolean));
        m.insert("NULL", Asn1TypeKind::Builtin(Asn1BuiltinType::Null));
        m.insert(
            "BIT-STRING",
            Asn1TypeKind::Builtin(Asn1BuiltinType::BitString),
        );
        m.insert(
            "OCTET-STRING",
            Asn1TypeKind::Builtin(Asn1BuiltinType::OctetString),
        );
        m.insert(
            "OBJECT-IDENTIFIER",
            Asn1TypeKind::Builtin(Asn1BuiltinType::ObjectIdentifier),
        );
        m.insert(
            "RELATIVE-OID",
            Asn1TypeKind::Builtin(Asn1BuiltinType::RelativeOid),
        );
        m.insert(
            "Printablestring",
            Asn1TypeKind::Builtin(Asn1BuiltinType::CharacterString),
        );
        m.insert(
            "IA5String",
            Asn1TypeKind::Builtin(Asn1BuiltinType::CharacterString),
        );
        m.insert(
            "UTF8String",
            Asn1TypeKind::Builtin(Asn1BuiltinType::CharacterString),
        );
        m
    };
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum Asn1BuiltinType {
    Integer(Asn1TypeInteger),
    Enumerated(Asn1TypeEnumerated),
    Boolean,
    Null,
    BitString,
    OctetString,
    ObjectIdentifier,
    RelativeOid,

    // Consumes a lot of String Types.
    CharacterString,
}

#[derive(Debug)]
pub(crate) struct Asn1Type {
    pub(crate) kind: Asn1TypeKind,
    pub(crate) constraints: Option<Vec<Asn1Constraint>>,
}

#[derive(Debug, Clone)]
pub(crate) enum Asn1TypeKind {
    Builtin(Asn1BuiltinType),
    Reference(Asn1TypeReference),
    Constructed(Asn1ConstructedType),
}

impl Default for Asn1TypeKind {
    fn default() -> Self {
        Self::Reference(Asn1TypeReference::Internal)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum Asn1TypeReference {
    External,
    ClassField,
    Internal,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum Asn1ConstructedType {
    Choice,
    Sequence,
    SequenceOf,
    Set,
    SetOf,
}
