//! Structs for the resolved Base Types

#[derive(Debug)]
pub(crate) enum ResolvedConstructedType {
    Choice(Asn1ResolvedChoice),
}

#[derive(Debug, Default)]
pub(crate) struct Asn1ResolvedChoice;
