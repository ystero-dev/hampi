//! Structs for the resolved Base Types

use crate::resolver::asn::structs::types::Asn1ResolvedType;

#[derive(Debug, Clone)]
pub(crate) enum ResolvedConstructedType {
    Choice {
        name: Option<String>,
        root_components: Vec<ResolvedComponent>,
        additions: Option<Vec<ResolvedComponent>>,
    },
    Sequence {
        name: Option<String>,
        components: Vec<ResolvedSeqComponent>,
    },
    SequenceOf {
        name: Option<String>,
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
