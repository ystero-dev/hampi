#![allow(dead_code)]
//! Structs related to ASN.1 Compiler
use std::cell::RefCell;
use std::collections::HashMap;

use crate::error::Error;
use crate::structs::parser::defs::{
    Asn1AssignmentKind, Asn1Definition, Asn1TypeAssignment, Asn1ValueAssignment,
};
use crate::structs::parser::module::Asn1Module;
use crate::structs::parser::types::{Asn1BuiltinType, Asn1TypeKind};
use crate::structs::resolver::{
    base::{Asn1ResolvedInteger, ResolvedBaseType},
    Asn1ResolvedDefinition, Asn1ResolvedType,
};

#[derive(Debug)]
pub struct Asn1Compiler {
    /// Modules belonging to this 'invocation' of compiler.
    modules: HashMap<String, RefCell<Asn1Module>>,
    all_definitions: HashMap<String, Asn1ResolvedDefinition>,
}

impl Asn1Compiler {
    pub fn new() -> Self {
        Asn1Compiler {
            modules: HashMap::new(),
            all_definitions: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, module: Asn1Module) -> bool {
        eprintln!("Adding Module!");
        let old = self
            .modules
            .insert(module.name.name.clone(), RefCell::new(module));
        !old.is_none()
    }

    pub fn resolve_imports(&self) -> Result<bool, Error> {
        for (_, cell) in self.modules.iter() {
            let module = cell.borrow();
            for (import, module_name) in module.imports.iter() {
                let target = self.modules.get(&module_name.name);
                if target.is_none() {
                    return Err(resolve_error!(
                        "Module '{}', corresponding to definition '{}' not found!",
                        module_name.name,
                        import
                    ));
                }
            }
        }
        eprintln!("All IMPORTS in All Modules Resolved!");
        Ok(true)
    }

    pub fn resolve_definitions(&mut self) -> Result<(), Error> {
        for cell in self.modules.values() {
            let mut module = cell.borrow_mut();
            let module_definitions = module.definitions_mut();
            for (k, def) in module_definitions.iter_mut() {
                let definition = self.resolve_definition(def)?;
                self.all_definitions.insert(k.clone(), definition);
            }
        }
        Ok(())
    }

    fn resolve_definition(
        &self,
        definition: &mut Asn1Definition,
    ) -> Result<Asn1ResolvedDefinition, Error> {
        match definition.kind {
            Asn1AssignmentKind::Value(ref v) => self.resolve_value_definition(v),
            Asn1AssignmentKind::Type(ref t) => self.resolve_type_definition(t),
            _ => Err(resolve_error!("Not Implemented!")),
        }
    }

    fn resolve_type_definition(
        &self,
        ty: &Asn1TypeAssignment,
    ) -> Result<Asn1ResolvedDefinition, Error> {
        eprintln!("{:#?}", ty);
        Err(resolve_error!("Resolve Type Definition: Not Implemented!"))
    }

    fn resolve_value_definition(
        &self,
        value: &Asn1ValueAssignment,
    ) -> Result<Asn1ResolvedDefinition, Error> {
        eprintln!("{:#?}", value);
        Err(resolve_error!("Resolve Value Definition: Not Implemented!"))
    }
}
