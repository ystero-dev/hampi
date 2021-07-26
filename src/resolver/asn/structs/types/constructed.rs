//! Structs for the resolved Base Types

#[derive(Debug, Clone)]
pub(crate) enum ResolvedConstructedType {
    Choice(Asn1ResolvedChoice),
    Sequence(Asn1ResolvedSequence),
    SequenceOf(Asn1ResolvedSequenceOf),
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedChoice;

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedSequence;

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedSequenceOf;
