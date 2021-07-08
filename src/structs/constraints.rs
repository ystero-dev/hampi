//! Related to handling of ASN.1 Constraints

#[derive(Debug)]
pub struct RangeElement {
    pub lower: String,
    pub lower_inclusive: bool,
    pub upper: String,
    pub upper_inclusive: bool,
}

#[derive(Debug)]
pub struct ValueElement {
    pub value: String,
}

#[derive(Debug)]
pub struct TypeElement {
    pub reference: String,
}

#[derive(Debug)]
pub struct SizeElement {
    pub values: UnionSet,
}

#[derive(Debug)]
pub enum Elements {
    Subtype(SubtypeElements),
    ElementSet(UnionSet),
}

#[derive(Debug)]
pub enum SubtypeElements {
    SingleValue(ValueElement),
    ConstrainedSubtype(TypeElement),
    ValueRange(RangeElement),
    SizeConstraint(SizeElement),
}

#[derive(Debug)]
pub struct UnionSet {
    pub elements: Vec<IntersectionSet>,
}

#[derive(Debug)]
pub struct IntersectionSet {
    pub elements: Vec<Elements>,
}

#[derive(Debug)]
pub struct Asn1Constraint {
    pub root_elements: UnionSet,
    pub additional_elements: Option<UnionSet>,
}
