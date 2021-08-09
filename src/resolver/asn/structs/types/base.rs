//! Structs for the resolved Base Types
use std::collections::{BTreeSet, HashMap};

use crate::resolver::asn::structs::types::constraints::Asn1ConstraintValueSet;

#[derive(Debug, Clone)]
pub(crate) enum ResolvedBaseType {
    Integer(Asn1ResolvedInteger),
    Enum(Asn1ResolvedEnumerated),
    BitString(Asn1ResolvedBitString),
    Boolean(Asn1ResolvedBoolean),
    OctetString(Asn1ResolvedOctetString),
    CharacterString(Asn1ResolvedCharacterString),
    ObjectIdentifier(Asn1ResolvedObjectIdentifier),
}

// An intermediate representation for a Resolved Integer Type
//
// This structure is obtained when all the 'Constraints' in a give definition are applied.
// Information from this structure can be directly used for code generation.
#[derive(Debug, Clone)]
pub(crate) struct Asn1ResolvedInteger {
    pub(crate) bytes: u8,
    pub(crate) signed: bool,
    pub(crate) resolved_constraints: Option<Asn1ConstraintValueSet>,
    pub(crate) named_values: Option<HashMap<String, i128>>,
}

impl Default for Asn1ResolvedInteger {
    fn default() -> Self {
        Self {
            bytes: 8,
            signed: true,
            named_values: None,
            resolved_constraints: None,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1ResolvedEnumerated {
    pub(crate) bytes: u8,
    pub(crate) signed: bool,
    pub(crate) values: Option<BTreeSet<i128>>,
    pub(crate) excepts: Option<BTreeSet<i128>>,
    pub(crate) named_values: HashMap<String, i128>,
}

impl Default for Asn1ResolvedEnumerated {
    fn default() -> Self {
        Self {
            bytes: 8,
            signed: true,
            values: None,
            excepts: None,
            named_values: HashMap::new(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedBitString;

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedBoolean;

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedOctetString {
    pub(crate) size: Option<Asn1ConstraintValueSet>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedCharacterString {
    pub(crate) size: Option<Asn1ConstraintValueSet>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedObjectIdentifier;
