//! ASN.1 Definitions relates Structs

use crate::structs::module::Asn1ModuleName;

#[derive(Debug)]
pub struct Asn1Constraint;

#[derive(Debug)]
pub struct Asn1TypeAssignment {
    /// Type Identifier
    pub id: String,

    /// Name following the assignment
    pub base: String,

    /// Constraints applied to the type
    pub constraints: Vec<Asn1Constraint>,

    /// Is Parameterized?
    pub is_parameterized: bool,

    /// Module to which this type belongs?
    pub module_name: Asn1ModuleName,

    /// Type definition text (concatenated text of all tokens that define this type.)
    pub definition: String,
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
pub struct Asn1ValueAssignment {
    /// Identifier for the value
    pub id: String,

    /// Type Reference
    pub typeref: String,

    /// Value Text
    pub value: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Asn1Definition {
    Value(Asn1ValueAssignment),
    Type(Asn1TypeAssignment),
    //ValueSetDefinition,
    //InfoObjectClassDefinition,
    //InfoObjectDefinition,
    //InfoObjectSetDefinition,
}

impl Asn1Definition {
    pub fn id(&self) -> String {
        match self {
            Self::Value(ref v) => v.id.clone(),
            Self::Type(ref t) => t.id.clone(),
        }
    }
}
