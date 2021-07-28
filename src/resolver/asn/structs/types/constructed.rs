//! Structs for the resolved Base Types

use crate::resolver::asn::structs::types::Asn1ResolvedType;

#[derive(Debug, Clone)]
pub(crate) enum ResolvedConstructedType {
    Choice(Asn1ResolvedChoice),
    Sequence(Asn1ResolvedSequence),
    SequenceOf(Asn1ResolvedSequenceOf),
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1ResolvedChoice {
    pub(crate) components: Vec<ResolvedComponent>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedSequence {
    pub(crate) components: Vec<ResolvedSeqComponent>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Asn1ResolvedSequenceOf;

#[derive(Debug, Clone)]
pub(crate) struct ResolvedComponent {
    pub(crate) id: String,
    pub(crate) ty: Asn1ResolvedType,
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedSeqComponent {
    pub(crate) component: ResolvedComponent,
    pub(crate) optional: bool,
    // FIXME : Handle Default
    // pub(crate) default: Option<Asn1ResolvedType>,
}
