//! Structures related to ASN.1 Type

pub(crate) mod base;
use base::{Asn1TypeBitString, Asn1TypeEnumerated, Asn1TypeInteger};

pub(crate) mod constraints;
use constraints::Asn1Constraint;

pub(crate) mod constructed;
use constructed::{Asn1TypeChoice, Asn1TypeSequence, Asn1TypeSequenceOf};

pub(crate) mod ioc;

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

impl Asn1Type {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        let mut kind_references = match self.kind {
            Asn1TypeKind::Builtin(ref _b) => vec![],
            Asn1TypeKind::Reference(ref r) => r.dependent_references(),
            Asn1TypeKind::Constructed(ref c) => c.dependent_references(),
        };

        // TODO: Constraint references
        let mut constraint_references: Vec<String> = vec![];
        kind_references.append(&mut constraint_references);
        kind_references
    }
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

#[derive(Debug, Clone)]
pub(crate) enum ActualParam {
    Set(String),
    Single(String),
}

impl ActualParam {
    fn dependent_references(&self) -> Vec<String> {
        match self {
            Self::Set(ref s) => vec![s.clone()],
            Self::Single(ref s) => vec![s.clone()],
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Asn1TypeReference {
    Reference(String),
    ClassField {
        classref: String,
        fieldref: String,
    },
    Parameterized {
        typeref: String,
        params: Vec<ActualParam>,
    }, // FIXME: For now We can make it a struct
}

impl Asn1TypeReference {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        match self {
            Self::ClassField { classref, .. } => vec![classref.clone()],
            Self::Reference(ref r) => vec![r.clone()],
            Self::Parameterized { typeref, params } => {
                let mut dependent_references = vec![typeref.clone()];
                for param in params {
                    dependent_references.extend(param.dependent_references());
                }
                dependent_references
            }
        }
    }
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

impl Asn1ConstructedType {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        match self {
            Self::Choice(ref c) => c.dependent_references(),
            Self::Sequence(ref s) => s.dependent_references(),
            Self::SequenceOf(ref so) => so.dependent_references(),
            _ => vec![],
        }
    }
}
