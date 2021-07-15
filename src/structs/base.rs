//! Handling of Integer Type

#[derive(Debug, Clone)]
pub enum NamedValue {
    Number(String),
    ValueRef(String),
}

/// A Structure representing ASN.1 Integer
#[derive(Debug, Clone)]
pub(crate) struct Asn1TypeInteger {
    pub(crate) named_values: Option<Vec<(String, NamedValue)>>,
}
