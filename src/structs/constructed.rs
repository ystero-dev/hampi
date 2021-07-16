//! Structures Representing Constructed Types

use super::constraints::Asn1Constraint;
use super::types::Asn1Type;

#[derive(Debug, Clone)]
pub(crate) struct Component {
    pub(crate) id: String,
    pub(crate) ty: Asn1Type,
}

#[derive(Debug, Clone)]
pub(crate) struct SeqComponent {
    pub(crate) component: Component,
    pub(crate) optional: bool,
    pub(crate) default: Option<String>, // FIXME: Should be Asn1Value later on
}

#[derive(Debug, Clone)]
pub(crate) struct SeqAdditionGroup {
    pub(crate) version: Option<String>,
    pub(crate) components: Vec<SeqComponent>,
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1TypeSequence {
    pub(crate) root_components: Vec<SeqComponent>,
    pub(crate) additions: Vec<SeqAdditionGroup>,
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1TypeChoice {
    pub(crate) components: Vec<Component>,
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1TypeSequenceOf {
    pub(crate) size: Option<Asn1Constraint>,
    pub(crate) ty: Box<Asn1Type>,
}
