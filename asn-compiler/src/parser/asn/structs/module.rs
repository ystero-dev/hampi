//! ASN.1 Module Level Structures and other functionality

use std::collections::{hash_map::Iter, HashMap};

use topological_sort::TopologicalSort;

use crate::parser::asn::structs::{defs::Asn1Definition, oid::ObjectIdentifier};

#[derive(Debug, PartialEq)]
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
    pub(in crate::parser) name: String,
    pub(in crate::parser) oid: Option<ObjectIdentifier>,
}

impl Asn1ModuleName {
    pub fn new(name: String, oid: Option<ObjectIdentifier>) -> Self {
        Self { name, oid }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn name_as_str(&self) -> &str {
        &self.name
    }

    pub fn oid(&self) -> Option<ObjectIdentifier> {
        self.oid.clone()
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
    pub(in crate::parser) imports: HashMap<String, Asn1ModuleName>,
    pub(in crate::parser) _exports: Option<Vec<Asn1Definition>>,
    pub(in crate::parser) name: Asn1ModuleName,
    pub(in crate::parser) _tags: Asn1ModuleTag,
    pub(in crate::parser) definitions: HashMap<String, Asn1Definition>,
    pub(in crate::parser) _exports_all: bool,
}

impl Asn1Module {
    pub fn name(self, name: Asn1ModuleName) -> Self {
        Self { name, ..self }
    }

    pub fn tags(self, _tags: Asn1ModuleTag) -> Self {
        Self { _tags, ..self }
    }

    pub fn imports(self, imports: HashMap<String, Asn1ModuleName>) -> Self {
        Self { imports, ..self }
    }

    pub(crate) fn definitions(self, definitions: HashMap<String, Asn1Definition>) -> Self {
        Self {
            definitions,
            ..self
        }
    }

    // Get all the definitions in the current module in the Topologically Sorted order.
    //
    // To 'sort' the definitions topologically, we are finding out all the 'dependents' on a given
    // definition (ie. those that are 'Referenced' in a given definition (like a Referenced Type in
    // a Type definition or a Class Name in an object definition and so on. We don't include the
    // 'import'ed definitions in these because, they will already be 'Resolved' because the modules
    // are already in a Topologically sorted order!
    pub(crate) fn definitions_sorted(&mut self) -> Vec<String> {
        let mut ts = TopologicalSort::<String>::new();

        for (k, v) in self.definitions.iter() {
            for r in v.dependent_references() {
                if self.imports.get(&r).is_none() {
                    ts.add_dependency(r.clone(), k.clone());
                }
            }
            ts.insert(k);
        }

        let mut out_vec = vec![];
        loop {
            let popped = ts.pop_all();
            if popped.is_empty() {
                break;
            } else {
                out_vec.extend(popped);
            }
        }
        out_vec
    }

    #[inline(always)]
    pub(crate) fn get_module_name(&self) -> String {
        self.name.name.clone()
    }

    // FIXME: Add filtering criteria
    #[inline(always)]
    pub(crate) fn get_definitions(&self) -> &HashMap<String, Asn1Definition> {
        &self.definitions
    }

    #[inline(always)]
    pub(crate) fn get_definition_mut(&mut self, key: &str) -> Option<&mut Asn1Definition> {
        self.definitions.get_mut(key)
    }

    #[inline(always)]
    pub(crate) fn get_imported_defs(&self) -> Iter<'_, String, Asn1ModuleName> {
        self.imports.iter()
    }
}
