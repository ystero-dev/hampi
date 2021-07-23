//! Structs for the resolved Base Types

#[derive(Debug, Clone)]
pub(crate) enum ResolvedConstructedType {
    Choice(Asn1ResolvedChoice),
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedChoice;
