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
pub(crate) enum AdditionGroupOrComponent {
    Component(SeqComponent),
    AdditionGroup(SeqAdditionGroup),
}

impl AdditionGroupOrComponent {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        match self {
            Self::Component(ref c) => c.dependent_references(),
            Self::AdditionGroup(ref g) => g.dependent_references(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SeqAdditionGroup {
    pub(crate) _version: Option<String>,
    pub(crate) components: Vec<SeqComponent>,
}

impl SeqAdditionGroup {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        self.components
            .iter()
            .flat_map(|c| c.dependent_references())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ChoiceAdditionGroup {
    pub(crate) _version: Option<String>,
    pub(crate) components: Vec<Component>,
}

impl ChoiceAdditionGroup {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        self.components
            .iter()
            .flat_map(|c| c.dependent_references())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1TypeSequence {
    pub(crate) root_components: Vec<SeqComponent>,
    pub(crate) additions: Vec<AdditionGroupOrComponent>,
    pub(crate) extensible: bool,
}

impl Asn1TypeSequence {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        let mut dependencies = vec![];
        for c in &self.root_components {
            dependencies.extend(c.dependent_references());
        }
        for a in &self.additions {
            dependencies.extend(a.dependent_references());
        }
        dependencies
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1TypeChoice {
    pub(crate) root_components: Vec<Component>,
    pub(crate) additions: Option<Vec<ChoiceAdditionGroup>>,
}

impl Asn1TypeChoice {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        let mut dependencies = vec![];
        for c in &self.root_components {
            dependencies.extend(c.dependent_references());
        }
        if self.additions.is_some() {
            for a in self.additions.as_ref().unwrap() {
                dependencies.extend(a.dependent_references());
            }
        }
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
        let mut output = vec![];
        if self.size.is_some() {
            let size = self.size.as_ref().unwrap().clone();
            output.extend(size.dependent_references());
        }
        output.extend(self.ty.dependent_references());
        output
    }
}
