//! Related to handling of ASN.1 Constraints

use crate::parser::asn::structs::oid::ObjectIdentifier;

use super::Asn1Type;

#[derive(Debug, Clone)]
pub(crate) enum SubtypeElements {
    SingleValue {
        value: String,
    },
    ConstrainedSubtype(Asn1Type),
    ValueRange {
        lower: String,
        lower_inclusive: bool,
        upper: String,
        upper_inclusive: bool,
    },
    SizeConstraint(ElementSet),
    PermittedAlphabet(ElementSet),
}

#[derive(Debug, Clone)]
pub(crate) enum Elements {
    Subtype(SubtypeElements),
    Set(ElementSet),
}
#[derive(Debug, Clone)]
pub(crate) struct IntersectionSet {
    pub(crate) elements: Vec<Elements>,
}

#[derive(Debug, Clone)]
pub(crate) struct UnionSet {
    pub(crate) elements: Vec<IntersectionSet>,
}

#[derive(Debug, Clone)]
pub(crate) struct ElementSet {
    pub(crate) root_elements: UnionSet,

    // Even if "empty" additinal_elements is present, this ElementSet is extensible.
    pub(crate) additional_elements: Option<UnionSet>,
}

// Related To Table Constraints
#[derive(Debug, Clone)]
pub(crate) enum ObjectSet {
    DefinedObjectSet(String),
}

// A Table Constraint is either a Simple Constraint (ObjectSet) or a ComponentRelation(@Components)
#[derive(Debug, Clone)]
pub(crate) enum TableConstraint {
    Simple(ObjectSet),
    ComponentRelation { table: String, component: String },
}

#[derive(Debug, Clone)]
pub(crate) enum Asn1Constraint {
    Subtype(ElementSet),
    Table(TableConstraint),
    Contents {
        containing: String,                   // Reference to referenced type
        encoded_by: Option<ObjectIdentifier>, // Encoding Object Identifier (Right now always None)
    },
}
