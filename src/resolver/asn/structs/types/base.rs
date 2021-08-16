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
    Null,
}

// An intermediate representation for a Resolved Integer Type
//
// This structure is obtained when all the 'Constraints' in a give definition are applied.
// Information from this structure can be directly used for code generation.
#[derive(Debug, Clone)]
pub(crate) struct Asn1ResolvedInteger {
    pub(crate) bits: u8,
    pub(crate) signed: bool,
    pub(crate) resolved_constraints: Option<Asn1ConstraintValueSet>,
    pub(crate) named_values: Option<HashMap<String, i128>>,
}

impl Default for Asn1ResolvedInteger {
    fn default() -> Self {
        Self {
            bits: 64,
            signed: true,
            named_values: None,
            resolved_constraints: None,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1ResolvedEnumerated {
    pub(crate) bits: u8,
    pub(crate) signed: bool,
    pub(crate) values: Option<BTreeSet<i128>>,
    pub(crate) excepts: Option<BTreeSet<i128>>,
    pub(crate) named_values: Vec<(String, i128)>,
}

impl Default for Asn1ResolvedEnumerated {
    fn default() -> Self {
        Self {
            bits: 8,
            signed: true,
            values: None,
            excepts: None,
            named_values: vec![],
        }
    }
}

// A Resolved `BIT STRING` representation. Normally only the `SIZE` Constraint needs to be
// resolved. If optional, `named_bits` are present, we maintain those in the map below.
#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedBitString {
    pub(crate) size: Option<Asn1ConstraintValueSet>,

    // We support only up to 128 named bits, if more than that is required, change this to appropriate. value
    pub(crate) named_values: HashMap<String, u8>,
}

// Just an empty structure for Resolved `BOOLEAN` type.
#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedBoolean;

// A structure representing a Resolved `OCTET STRING`. `SIZE` Constraint is resolved as well. The
// `CONTAINING` Constraint is not resolved.
#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedOctetString {
    pub(crate) size: Option<Asn1ConstraintValueSet>,
}

// A structure representing a Resolved `CharacterString`. `SIZE` Constraint is resolved as well. The
#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedCharacterString {
    pub(crate) size: Option<Asn1ConstraintValueSet>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedObjectIdentifier;
