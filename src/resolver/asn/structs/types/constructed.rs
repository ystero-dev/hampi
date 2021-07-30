//! Structs for the resolved Base Types

use crate::resolver::asn::structs::types::Asn1ResolvedType;

#[derive(Debug, Clone)]
pub(crate) enum ResolvedConstructedType {
    Choice {
        components: Vec<ResolvedComponent>,
    },
    Sequence {
        components: Vec<ResolvedSeqComponent>,
    },
    SequenceOf {
        ty: Box<Asn1ResolvedType>,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedComponent {
    pub(crate) id: String,
    pub(crate) ty: Asn1ResolvedType,
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedSeqComponent {
    pub(crate) component: ResolvedComponent,
    pub(crate) optional: bool,
    // FIXME : Handle default
    // pub(crate) default: Option<Asn1ResolvedType>,
}
