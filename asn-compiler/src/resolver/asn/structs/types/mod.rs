use std::collections::BTreeMap;

use crate::parser::asn::structs::types::Asn1TagClass;

pub(crate) mod constructed;
use constructed::ResolvedConstructedType;

pub(crate) mod base;
use base::ResolvedBaseType;

pub(crate) mod ioc;

pub(crate) mod constraints;

pub(crate) type ResolvedSetTypeMap = BTreeMap<(String, String), (String, Asn1ResolvedType)>;

#[derive(Debug, Clone)]
pub(crate) struct ResolvedSetType {
    pub(crate) setref: String,
    pub(crate) types: ResolvedSetTypeMap,
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

// When tags are to be supported, an instance of this class will be available for each of the
// 'resolved' type.
#[derive(Debug, Clone, Default)]
pub(crate) struct Asn1ResolvedTag {
    num: u32,
    class: Asn1TagClass,
}
