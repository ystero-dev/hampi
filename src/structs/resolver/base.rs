//! Structs for the resolved Base Types
use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
pub(crate) enum ResolvedBaseType {
    Integer(Asn1ResolvedInteger),
    Enum(Asn1ResolvedEnumerated),
}

// An intermediate representation for a Resolved Integer Type
//
// This structure is obtained when all the 'Constraints' in a give definition are applied.
// Information from this structure can be directly used for code generation.
#[derive(Debug)]
pub(crate) struct Asn1ResolvedInteger {
    bytes: u8,
    signed: bool,
    values: Option<BTreeSet<i64>>,
    excepts: Option<BTreeSet<i64>>,
    named_values: Option<HashMap<String, i64>>,
}

impl Default for Asn1ResolvedInteger {
    fn default() -> Self {
        Self {
            bytes: 8,
            signed: true,
            values: None,
            excepts: None,
            named_values: None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Asn1ResolvedEnumerated {
    named_values: HashMap<String, i64>,
}
