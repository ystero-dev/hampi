//! Structures related to ASN.1 Type

#[allow(dead_code)]
#[derive(Debug)]
pub enum Asn1BuiltInType {
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
    UnResolved,
}

impl Default for Asn1BuiltInType {
    fn default() -> Self {
        Self::UnResolved
    }
}

#[derive(Default)]
pub struct Asn1Type {
    pub kind: Asn1BuiltInType,
    pub id: String,
}
