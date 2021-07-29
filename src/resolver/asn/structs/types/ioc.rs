//! Resolved(Object|ObjectSet) Definitions
use std::collections::HashMap;

use crate::resolver::asn::structs::{types::Asn1ResolvedType, values::Asn1ResolvedValue};

#[derive(Debug, Clone)]
pub(crate) enum ResolvedFieldSpec {
    Type {
        ty: Option<Asn1ResolvedType>,
    },
    FixedTypeValue {
        typeref: Asn1ResolvedType,
        value: Option<Asn1ResolvedValue>,
    },
    Unresolved, // Only So that we can create a 'Default' value for the timebeing
}

impl Default for ResolvedFieldSpec {
    fn default() -> Self {
        Self::Unresolved
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedObject {
    pub(crate) fields: HashMap<String, ResolvedFieldSpec>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedObjectSet {
    pub(crate) objects: ResolvedObjectSet,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct ResolvedObjectSet {
    pub(crate) elements: Vec<ResolvedObjectSetElement>,
}

#[derive(Debug, Clone)]
pub(crate) enum ResolvedObjectSetElement {
    ObjectSetReference(String),
    ObjectReference(String),
    Object(Asn1ResolvedObject),
}
