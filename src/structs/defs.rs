//! ASN.1 Definitions relates Structs

use super::ioc::{Asn1ObjectClass, Asn1ObjectSet};
use super::types::Asn1Type;

/// Struct representing an Object Class Assignment
#[derive(Debug)]
pub(crate) struct Asn1ObjectClassAssignment {
    /// Class Identifier
    pub(crate) id: String,

    /// Definition
    pub(crate) classref: Asn1ObjectClass,
}

/// Struct representing a Object Set Assignment
#[derive(Debug)]
pub(crate) struct Asn1ObjectSetAssignment {
    /// Identifier
    pub(crate) id: String,

    pub(crate) set: Asn1ObjectSet,
}

#[derive(Debug)]
pub(crate) struct Asn1TypeAssignment {
    /// Type Identifier
    pub(crate) id: String,

    /// Type Referred to on the RHS Side of Assignment
    pub(crate) typeref: Asn1Type,
}

/// A Value Assignment in ASN.1 Module
///
/// A Value Assignment in ASN.1 module looks like -
///
/// ```asn
///
///     maxnoofElements INTEGER ::= 1000
///
/// ```
///
/// A Value assginement resolves to the `id` will be used as a key when looking up for
/// values in a given module. The 'resolved' type of the value. See [`ResolvedType`]. Actual value
/// (refering to a value of base ASN.1 Type. See [`base_types`]). Note: When the value is
/// 'resolved', the actual value contains one of the base type values after all constraints are
/// validated.
#[derive(Debug)]
pub(crate) struct Asn1ValueAssignment {
    /// Identifier for the value
    pub(crate) id: String,

    /// Type Reference
    pub(crate) typeref: Asn1Type,

    /// Value Text
    pub(crate) value: String,
}

#[derive(Debug)]
pub(crate) enum Asn1AssignmentKind {
    Value(Asn1ValueAssignment),
    Type(Asn1TypeAssignment),
    Class(Asn1ObjectClassAssignment),
    ObjectSet(Asn1ObjectSetAssignment),
    //InfoObjectClassDefinition,
    //InfoObjectDefinition,
    //InfoObjectSetDefinition,
}

impl Asn1AssignmentKind {
    pub fn id(&self) -> String {
        match self {
            Self::Value(ref v) => v.id.clone(),
            Self::Type(ref t) => t.id.clone(),
            Self::Class(ref c) => c.id.clone(),
            Self::ObjectSet(ref s) => s.id.clone(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Asn1Definition {
    pub(crate) kind: Asn1AssignmentKind,
    pub(crate) params: Option<Vec<DefinitionParam>>,
}

// FIXME: Hack for now
impl Asn1Definition {
    pub fn id(&self) -> String {
        self.kind.id()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum GovernerKind {
    Type,
    Class,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ParamGoverner {
    pub(crate) name: String,
    pub(crate) kind: GovernerKind,
}

#[derive(Debug)]
pub(crate) enum DummyReferenceKind {
    Type,
    Value,
    ValueSet,
    Class,
    Object,
    ObjectSet,
}

#[derive(Debug)]
pub(crate) struct ParamDummyReference {
    pub(crate) name: String,
    pub(crate) kind: DummyReferenceKind,
}

#[derive(Debug)]
pub(crate) struct DefinitionParam {
    pub(crate) governer: Option<ParamGoverner>,
    pub(crate) dummyref: ParamDummyReference,
}
