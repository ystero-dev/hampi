//! ASN.1 Definitions related Structs

use super::types::{
    ioc::{Asn1Object, Asn1ObjectClass, Asn1ObjectSet},
    Asn1Type,
};

/// Struct representing an Object Class Assignment
#[derive(Debug, Clone)]
pub(crate) struct Asn1ObjectClassAssignment {
    /// Class Identifier
    pub(crate) id: String,

    /// Definition
    pub(crate) classref: Asn1ObjectClass,
}

impl Asn1ObjectClassAssignment {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        self.classref.dependent_references()
    }
}

/// Struct representing Object Set Assignment
#[derive(Debug, Clone)]
pub(crate) struct Asn1ObjectSetAssignment {
    /// Identifier
    pub(crate) id: String,

    pub(crate) set: Asn1ObjectSet,
}

impl Asn1ObjectSetAssignment {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        self.set.dependent_references()
    }
}

/// Struct representing an Object Assignment
#[derive(Debug, Clone)]
pub(crate) struct Asn1ObjectAssignment {
    ///Identifier
    pub(crate) id: String,
    pub(crate) object: Asn1Object,
}

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub(crate) struct Asn1ValueAssignment {
    /// Identifier for the value
    pub(crate) id: String,

    /// Type Reference
    pub(crate) typeref: Asn1Type,

    /// Value Text
    pub(crate) value: String,
}

#[derive(Debug, Clone)]
pub(crate) enum Asn1AssignmentKind {
    Value(Asn1ValueAssignment),
    Type(Asn1TypeAssignment),
    Class(Asn1ObjectClassAssignment),
    ObjectSet(Asn1ObjectSetAssignment),
    Object(Asn1ObjectAssignment),
}

impl Asn1AssignmentKind {
    pub fn id(&self) -> String {
        match self {
            Self::Value(ref v) => v.id.clone(),
            Self::Type(ref t) => t.id.clone(),
            Self::Class(ref c) => c.id.clone(),
            Self::ObjectSet(ref s) => s.id.clone(),
            Self::Object(ref o) => o.id.clone(),
        }
    }

    pub fn dependent_references(&self) -> Vec<String> {
        match self {
            Self::Value(ref v) => v.typeref.dependent_references(),
            Self::Type(ref t) => t.typeref.dependent_references(),
            Self::Object(ref o) => vec![o.object.class.clone()],
            Self::ObjectSet(ref s) => s.dependent_references(),
            Self::Class(ref c) => c.dependent_references(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1Definition {
    pub(crate) kind: Asn1AssignmentKind,
    pub(crate) params: Option<Vec<DefinitionParam>>,
    pub(crate) resolved: bool,
}

// FIXME: Hack for now
impl Asn1Definition {
    pub fn id(&self) -> String {
        self.kind.id()
    }
    // Returns a list of dependent references for a given defintion. These will be used to sort the
    // definitions topologically, which would make error handling considerably easier when
    // resolving those definitions
    pub fn dependent_references(&self) -> Vec<String> {
        self.kind.dependent_references()
    }
}

macro_rules! is_assignment_kind {

    (($fn:ident, $variant: path)) => {
        #[allow(dead_code)]
        impl Asn1Definition {
            pub fn $fn(&self) -> bool {
                if let $variant(ref _x) = self.kind {
                    true
                } else {
                false
                }
            }
        }
    };

    ($($tt:tt,)*) => {
        $(
        is_assignment_kind!($tt);
        )+
    };
}

is_assignment_kind! {
    (is_value_assignment, Asn1AssignmentKind::Value),
    (is_type_assignment, Asn1AssignmentKind::Type),
    (is_class_assignment, Asn1AssignmentKind::Class),
    (is_object_set_assignment, Asn1AssignmentKind::ObjectSet),
    (is_object_assignment, Asn1AssignmentKind::Object),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum GovernerKind {
    Type,
    Class,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct ParamGoverner {
    pub(crate) name: String,
    pub(crate) kind: GovernerKind,
}

#[derive(Debug, Clone)]
pub(crate) enum DummyReferenceKind {
    Type,
    Value,
    ValueSet,
    Class,
    Object,
    ObjectSet,
}

#[derive(Debug, Clone)]
pub(crate) struct ParamDummyReference {
    pub(crate) name: String,
    pub(crate) kind: DummyReferenceKind,
}

#[derive(Debug, Clone)]
pub(crate) struct DefinitionParam {
    pub(crate) governer: Option<ParamGoverner>,
    pub(crate) dummyref: ParamDummyReference,
}
