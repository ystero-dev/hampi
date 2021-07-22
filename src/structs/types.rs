//! Structures related to ASN.1 Type

use super::base::{Asn1TypeBitString, Asn1TypeEnumerated, Asn1TypeInteger};
use super::constraints::Asn1Constraint;
use super::constructed::{Asn1TypeChoice, Asn1TypeSequence, Asn1TypeSequenceOf};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum Asn1BuiltinType {
    Integer(Asn1TypeInteger),
    Enumerated(Asn1TypeEnumerated),
    BitString(Asn1TypeBitString),
    Boolean,
    Null,
    OctetString,
    ObjectIdentifier,
    RelativeOid,

    // Consumes a lot of String Types.
    CharacterString,
}

#[derive(Debug, Clone)]
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
        Self::Reference(Asn1TypeReference::Reference("".to_string()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum Asn1TypeReference {
    ClassField(String),
    Reference(String),
    Parameterized(String), // FIXME: For now We can make it a struct
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum Asn1ConstructedType {
    Choice(Asn1TypeChoice),
    Sequence(Asn1TypeSequence),
    SequenceOf(Asn1TypeSequenceOf),
    Set,
    SetOf,
}
