use super::types::ioc::{Asn1ResolvedObject, Asn1ResolvedObjectSet};
use super::types::Asn1ResolvedType;
use super::values::Asn1ResolvedValue;

#[derive(Debug, Clone)]
pub(crate) enum Asn1ResolvedDefinition {
    Type(Asn1ResolvedType),
    Value(Asn1ResolvedValue),
    ObjectSet(Asn1ResolvedObjectSet),
    Object(Asn1ResolvedObject),
}
