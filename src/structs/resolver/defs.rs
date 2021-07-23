use super::types::Asn1ResolvedType;
use super::values::Asn1ResolvedValue;

#[derive(Debug)]
pub(crate) enum Asn1ResolvedDefinition {
    Type(Asn1ResolvedType),
    Value(Asn1ResolvedValue),
}
