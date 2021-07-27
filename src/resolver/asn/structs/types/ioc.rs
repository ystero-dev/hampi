//! Resolved(Object|ObjectSet) Definitions

#[derive(Debug, Clone, Default)]
pub(crate) struct ResolvedFieldSpec;

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedObject {
    pub(crate) fields: Vec<ResolvedFieldSpec>,
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
