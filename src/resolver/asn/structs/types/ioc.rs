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
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1ResolvedObject {
    pub(crate) name: String,
    pub(crate) fields: HashMap<String, ResolvedFieldSpec>,
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1ResolvedObjectSet {
    pub(crate) objects: ResolvedObjectSet,
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedObjectSet {
    pub(crate) elements: Vec<ResolvedObjectSetElement>,
    pub(crate) lookup_table: HashMap<String, ResolvedObjectSetElement>,
}

#[derive(Debug, Clone)]
pub(crate) enum ResolvedObjectSetElement {
    ObjectSetReference(String),
    ObjectReference(String),
    Object(Asn1ResolvedObject),
}
