pub(crate) mod constructed;
use constructed::ResolvedConstructedType;

pub(crate) mod base;
use base::ResolvedBaseType;

pub(crate) mod ioc;

#[derive(Debug, Clone)]
pub(crate) enum Asn1ResolvedType {
    Base(ResolvedBaseType),
    Constructed(ResolvedConstructedType),
}
