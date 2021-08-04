use std::collections::HashMap;

pub(crate) mod constructed;
use constructed::ResolvedConstructedType;

pub(crate) mod base;
use base::ResolvedBaseType;

pub(crate) mod ioc;

#[derive(Debug, Clone)]
pub(crate) struct ResolvedSetType {
    pub(crate) setref: String,
    pub(crate) types: HashMap<String, Asn1ResolvedType>,
}

#[derive(Debug, Clone)]
pub(crate) enum Asn1ResolvedType {
    // One of the resolved Base Types.
    Base(ResolvedBaseType),

    // A Constructed Type with fields that are Asn1ResolvedType.
    Constructed(ResolvedConstructedType),

    // A reference to a Resolved Type
    Reference(String),

    // A Set of Resolved Types. This is true if the type is obtained from Object Sets or Value Sets
    Set(ResolvedSetType),
}
