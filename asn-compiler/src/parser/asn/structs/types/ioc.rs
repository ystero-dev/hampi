//! Structures related to Information Object Class, Objects etc.

use std::collections::HashMap;

use super::Asn1Type;

#[derive(Debug, Clone)]
pub(crate) struct FixedTypeValueFieldSpec {
    pub(crate) id: String,

    pub(crate) field_type: Asn1Type,
    pub(crate) unique: bool,
    pub(crate) default: Option<String>,
    pub(crate) optional: bool,
    pub(crate) with_syntax: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypeFieldSpec {
    pub(crate) id: String,
    pub(crate) optional: bool,
    pub(crate) default: Option<Asn1Type>,
    pub(crate) with_syntax: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) enum ObjectClassFieldSpec {
    Type {
        id: String,
        default: Option<Asn1Type>,
        optional: bool,
        with_syntax: Option<String>,
        resolved: bool,
    },
    FixedTypeValue {
        id: String,

        field_type: Asn1Type,
        unique: bool,
        default: Option<String>,
        optional: bool,
        with_syntax: Option<String>,
        resolved: bool,
    },
    // TODO: Following Field Specs are not implemented right now
    // VariableTypeValue(VariableTypeValueFieldSpec),
    // FixedTypeValueSet(FixedTypeValueSetFieldSpec),
    // VariableTypeValueSet(VariableTypeValueSetSpec),
    // Object(ObjectFieldSpec),
    // ObjectSet(ObjectSetFieldSpec)
}

impl ObjectClassFieldSpec {
    pub(crate) fn id(&self) -> String {
        match self {
            Self::Type { id, .. } | Self::FixedTypeValue { id, .. } => id.clone(),
        }
    }

    fn with_syntax(&self) -> Option<String> {
        match self {
            Self::Type { with_syntax, .. } | Self::FixedTypeValue { with_syntax, .. } => {
                with_syntax.clone()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1ObjectClass {
    pub(crate) fields: HashMap<String, ObjectClassFieldSpec>,
}

impl Asn1ObjectClass {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        let mut output = vec![];
        for field in self.fields.values() {
            if let ObjectClassFieldSpec::FixedTypeValue { field_type, .. } = field {
                let mut field_references = field_type.dependent_references();
                output.append(&mut field_references);
            }
        }
        output
    }

    pub(crate) fn get_first_unique_field_id(&self) -> Option<String> {
        for (name, value) in &self.fields {
            if let ObjectClassFieldSpec::FixedTypeValue { unique, .. } = value {
                if *unique {
                    return Some(name.clone());
                }
            }
        }
        None
    }

    pub(crate) fn get_with_syntax_words(&self) -> Vec<String> {
        self.fields
            .values()
            .filter_map(|v| v.with_syntax())
            .collect::<Vec<String>>()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1ObjectSet {
    pub(crate) class: String,      // Class for which this Object Set is defined
    pub(crate) objects: ObjectSet, // Actual Object Set
}

impl Asn1ObjectSet {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        let mut output = vec![self.class.clone()];
        output.append(&mut self.objects.dependent_references());
        output
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Asn1ObjectFieldSpec {
    Type {
        ty: Option<Asn1Type>,
    },
    FixedTypeValue {
        typeref: Asn1Type,
        value: Option<String>,
    },
}

impl Asn1ObjectFieldSpec {
    fn dependent_references(&self) -> Vec<String> {
        match self {
            Self::Type { ty } => {
                if ty.is_some() {
                    ty.as_ref().unwrap().dependent_references()
                } else {
                    vec![]
                }
            }
            Self::FixedTypeValue { typeref, .. } => typeref.dependent_references(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Asn1ObjectValue {
    Asn1ObjectFromClass {
        fields: HashMap<String, Asn1ObjectFieldSpec>,
    },
    Input(String),
}

impl Asn1ObjectValue {
    fn dependent_references(&self) -> Vec<String> {
        let mut output = vec![];
        if let Self::Asn1ObjectFromClass { fields } = self {
            for field in fields.values() {
                output.extend(field.dependent_references());
            }
        }
        output
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Asn1Object {
    pub(crate) class: String, // Class for which this Object Set is defined
    pub(crate) value: Asn1ObjectValue, // For now just a string,
}

#[derive(Debug, Clone)]
pub(crate) struct ObjectSet {
    pub(crate) root_elements: Vec<ObjectSetElement>,
    pub(crate) additional_elements: Vec<ObjectSetElement>,
}

impl ObjectSet {
    pub(crate) fn dependent_references(&self) -> Vec<String> {
        let mut output = vec![];
        for e in &self.root_elements {
            output.extend(e.dependent_references());
        }
        for e in &self.additional_elements {
            output.extend(e.dependent_references());
        }
        output
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ObjectSetElement {
    ObjectSetReference(String), // A Reference to a defined Object Set
    ObjectReference(String),    // A reference to a defined Object
    Object(Asn1ObjectValue),    // An object defined Inline
}

impl ObjectSetElement {
    fn dependent_references(&self) -> Vec<String> {
        match self {
            Self::ObjectSetReference(ref r) | Self::ObjectReference(ref r) => vec![r.clone()],
            Self::Object(ref o) => o.dependent_references(),
        }
    }
}
