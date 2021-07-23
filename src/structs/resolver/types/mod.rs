pub(crate) mod constructed;
use constructed::ResolvedConstructedType;

pub(crate) mod base;
use base::ResolvedBaseType;

#[derive(Debug)]
pub(crate) enum Asn1ResolvedType {
    Base(ResolvedBaseType),
    Constructed(ResolvedConstructedType),
}
