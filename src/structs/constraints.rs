//! Related to handling of ASN.1 Constraints

use super::oid::ObjectIdentifier;
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
pub(crate) struct ElementSetConstraint {
    pub(crate) root_elements: UnionSet,
    pub(crate) additional_elements: Option<UnionSet>,
}

#[derive(Debug, Clone)]
pub(crate) enum ObjectSet {
    DefinedObjectSet(String),
}

#[derive(Debug, Clone)]
pub(crate) struct ComponentRelation {
    pub(crate) table: String,     // Always a defined
    pub(crate) component: String, // Reference to @ component
}

#[derive(Debug, Clone)]
pub(crate) enum TableConstraint {
    Simple(ObjectSet),
    CompRel(ComponentRelation),
}

#[derive(Debug, Clone)]
pub(crate) struct ContentsConstraint {
    pub(crate) containing: String, // Reference to referenced type
    pub(crate) encodedby: Option<ObjectIdentifier>, // Encoding Object Identifier (Right now always None)
}

#[derive(Debug, Clone)]
pub(crate) enum Asn1Constraint {
    Subtype(ElementSetConstraint),
    Table(TableConstraint),
    Contents(ContentsConstraint),
}
