//! Related to handling of ASN.1 Constraints

use super::types::Asn1Type;

#[derive(Debug, Clone)]
pub(crate) struct RangeElement {
    pub(crate) lower: String,
    pub(crate) lower_inclusive: bool,
    pub(crate) upper: String,
    pub(crate) upper_inclusive: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ValueElement {
    pub(crate) value: String,
}

#[derive(Debug, Clone)]
pub(crate) struct UnionSetElement {
    pub(crate) values: UnionSet,
}

#[derive(Debug, Clone)]
pub(crate) enum Elements {
    Subtype(SubtypeElements),
    ElementSet(UnionSet),
}

#[derive(Debug, Clone)]
pub(crate) enum SubtypeElements {
    SingleValue(ValueElement),
    ConstrainedSubtype(Asn1Type),
    ValueRange(RangeElement),
    SizeConstraint(UnionSetElement),
    PermittedAlphabet(UnionSetElement),
}

#[derive(Debug, Clone)]
pub(crate) struct UnionSet {
    pub(crate) elements: Vec<IntersectionSet>,
}

#[derive(Debug, Clone)]
pub(crate) struct IntersectionSet {
    pub(crate) elements: Vec<Elements>,
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1Constraint {
    pub(crate) root_elements: UnionSet,
    pub(crate) additional_elements: Option<UnionSet>,
}
