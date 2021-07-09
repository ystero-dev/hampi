//! Structures related to ASN.1 Type

use super::constraints::Asn1Constraint;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Asn1BuiltInType {
    Integer,
    Enumerated,
    Boolean,
    Null,
    BitString,
    OctetString,
    ObjectIdentifier,
    RelativeOid,
    Sequence,
    Set,
    Choice,
    SequenceOf,
    SetOf,

    // Consumes a lot of String Types.
    CharacterString,
    // We don't know yet
    Unresolved,
}

impl Default for Asn1BuiltInType {
    fn default() -> Self {
        Self::Unresolved
    }
}

#[derive(Debug)]
pub(crate) struct Asn1Type {
    pub(crate) id: String,
    pub(crate) kind: Asn1BuiltInType,
    pub(crate) constraints: Option<Vec<Asn1Constraint>>,
}
