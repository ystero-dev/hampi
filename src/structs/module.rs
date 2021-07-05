//! ASN.1 Module Level Structures and other functionality

use std::collections::HashMap;

use crate::structs::{defs::Asn1Definition, oid::ObjectIdentifier};

#[derive(Debug)]
pub enum Asn1ModuleTag {
    Explicit,
    Implicit,
    Automatic,
}

impl Default for Asn1ModuleTag {
    fn default() -> Self {
        Self::Explicit
    }
}

#[derive(Debug, Default, Clone)]
pub struct Asn1ModuleName {
    name: String,
    oid: Option<ObjectIdentifier>,
}

impl Asn1ModuleName {
    pub fn new(name: String, oid: Option<ObjectIdentifier>) -> Self {
        Self { name, oid }
    }
}

/// Definition of a 'parsed' ASN Module
///
/// When an ASN module is successfully parsed, it contains a set of definitions that result from
/// parsing the assignments in a given module (Definitions within 'BEGIN' and 'END' tags.
/// Optionally, the module may have some definitions, that are imported from other modules and the
/// module may even 'export' some defintions. A module is uniquely identified by a name and object
/// identifier. In addition a module may support 'tagging' internal sequence values differently, so
/// information about it is kept as well.
#[derive(Debug, Default)]
pub struct Asn1Module {
    imports: HashMap<String, Asn1ModuleName>,
    exports: Option<Vec<Asn1Definition>>,
    name: Asn1ModuleName,
    tags: Asn1ModuleTag,
    definitions: HashMap<String, Asn1Definition>,
    exports_all: bool,
}

impl Asn1Module {
    pub fn name(self, name: Asn1ModuleName) -> Self {
        Self { name, ..self }
    }

    pub fn tags(self, tags: Asn1ModuleTag) -> Self {
        Self { tags, ..self }
    }

    pub fn imports(self, imports: HashMap<String, Asn1ModuleName>) -> Self {
        Self { imports, ..self }
    }

    pub fn definitions(self, definitions: HashMap<String, Asn1Definition>) -> Self {
        Self {
            definitions,
            ..self
        }
    }
}
