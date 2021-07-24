//! Resolved(Object|ObjectSet) Definitions

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedObject;

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedObjectSet {
    pub(crate) objects: Vec<Asn1ResolvedObject>,
}
