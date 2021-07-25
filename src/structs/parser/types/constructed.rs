//! Structures Representing Constructed Types

use super::constraints::Asn1Constraint;
use super::Asn1Type;

#[derive(Debug, Clone)]
pub(crate) struct Component {
    pub(crate) id: String,
    pub(crate) ty: Asn1Type,
}

impl Component {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        self.ty.dependent_references()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SeqComponent {
    pub(crate) component: Component,
    pub(crate) optional: bool,
    pub(crate) default: Option<String>, // FIXME: Should be Asn1Value later on
}

impl SeqComponent {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        self.component.dependent_references()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SeqAdditionGroup {
    pub(crate) version: Option<String>,
    pub(crate) components: Vec<SeqComponent>,
}

impl SeqAdditionGroup {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        let mut dependencies = vec![];
        let _ = self
            .components
            .iter()
            .map(|c| dependencies.append(&mut c.dependent_references()));
        dependencies
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ChoiceAdditionGroup {
    pub(crate) version: Option<String>,
    pub(crate) components: Vec<Component>,
}

impl ChoiceAdditionGroup {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        let mut dependencies = vec![];
        let _ = self
            .components
            .iter()
            .map(|c| dependencies.append(&mut c.dependent_references()));
        dependencies
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1TypeSequence {
    pub(crate) root_components: Vec<SeqComponent>,
    pub(crate) additions: Vec<SeqAdditionGroup>,
}

impl Asn1TypeSequence {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        let mut dependencies = vec![];
        let _ = self
            .root_components
            .iter()
            .map(|c| dependencies.append(&mut c.dependent_references()));
        let mut addition_dependencies = vec![];
        let _ = self
            .additions
            .iter()
            .map(|a| dependencies.append(&mut a.dependent_references()));
        dependencies.append(&mut addition_dependencies);
        dependencies
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1TypeChoice {
    pub(crate) components: Vec<Component>,
    pub(crate) additions: Vec<ChoiceAdditionGroup>,
}

impl Asn1TypeChoice {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        let mut dependencies = vec![];
        let _ = self
            .components
            .iter()
            .map(|c| dependencies.append(&mut c.dependent_references()));
        let mut addition_dependencies = vec![];
        let _ = self
            .additions
            .iter()
            .map(|a| dependencies.append(&mut a.dependent_references()));
        dependencies.append(&mut addition_dependencies);
        dependencies
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1TypeSequenceOf {
    pub(crate) size: Option<Asn1Constraint>,
    pub(crate) ty: Box<Asn1Type>,
}

impl Asn1TypeSequenceOf {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        self.ty.dependent_references()
    }
}
