//! ASN.1 Definitions relates Structs

use super::types::Asn1Type;

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

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Asn1Definition {
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
