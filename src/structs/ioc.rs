//! Structures related to Information Object Class, Objects etc.

use std::collections::HashMap;

use super::types::Asn1Type;

#[derive(Debug)]
pub(crate) struct FixedTypeValueFieldSpec {
    pub(crate) id: String,

    pub(crate) field_type: Asn1Type,
    pub(crate) unique: bool,
    pub(crate) default: Option<String>,
    pub(crate) optional: bool,
    pub(crate) with_syntax: Option<String>,
}

#[derive(Debug)]
pub(crate) struct TypeFieldSpec {
    pub(crate) id: String,
    pub(crate) optional: bool,
    pub(crate) default: Option<Asn1Type>,
    pub(crate) with_syntax: Option<String>,
}

#[derive(Debug)]
pub(crate) enum ObjectClassFieldSpec {
    Type(TypeFieldSpec),

    FixedTypeValue(FixedTypeValueFieldSpec),
    // TODO: Following Field Specs are not implemented right now
    // VariableTypeValue(VariableTypeValueFieldSpec),
    // FixedTypeValueSet(FixedTypeValueSetFieldSpec),
    // VariableTypeValueSet(VariableTypeValueSetSpec),
    // Object(ObjectFieldSpec),
    // ObjectSet(ObjectSetFieldSpec),
}

impl ObjectClassFieldSpec {
    pub(crate) fn id(&self) -> String {
        match self {
            Self::Type(t) => t.id.clone(),
            Self::FixedTypeValue(v) => v.id.clone(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Asn1ObjectClass {
    pub(crate) fields: HashMap<String, ObjectClassFieldSpec>,
}
