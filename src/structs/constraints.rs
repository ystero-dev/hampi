//! Related to handling of ASN.1 Constraints

#[derive(Debug)]
pub(crate) struct RangeElement {
    pub(crate) lower: String,
    pub(crate) lower_inclusive: bool,
    pub(crate) upper: String,
    pub(crate) upper_inclusive: bool,
}

#[derive(Debug)]
pub(crate) struct ValueElement {
    pub(crate) value: String,
}

#[derive(Debug)]
pub(crate) struct TypeElement {
    pub(crate) reference: String,
}

#[derive(Debug)]
pub(crate) struct UnionSetElement {
    pub(crate) values: UnionSet,
}

#[derive(Debug)]
pub(crate) enum Elements {
    Subtype(SubtypeElements),
    ElementSet(UnionSet),
}

#[derive(Debug)]
pub(crate) enum SubtypeElements {
    SingleValue(ValueElement),
    ConstrainedSubtype(TypeElement),
    ValueRange(RangeElement),
    SizeConstraint(UnionSetElement),
    PermittedAlphabet(UnionSetElement),
}

#[derive(Debug)]
pub(crate) struct UnionSet {
    pub(crate) elements: Vec<IntersectionSet>,
}

#[derive(Debug)]
pub(crate) struct IntersectionSet {
    pub(crate) elements: Vec<Elements>,
}

#[derive(Debug)]
pub(crate) struct Asn1Constraint {
    pub(crate) root_elements: UnionSet,
    pub(crate) additional_elements: Option<UnionSet>,
}
