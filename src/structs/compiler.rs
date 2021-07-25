#![allow(dead_code)]
//! Structs related to ASN.1 Compiler
use std::cell::RefCell;
use std::collections::HashMap;

use topological_sort::TopologicalSort;

use crate::error::Error;

use crate::structs::parser::{
    defs::{Asn1AssignmentKind, Asn1Definition, Asn1ObjectClassAssignment},
    module::Asn1Module,
};
use crate::structs::resolver::defs::Asn1ResolvedDefinition;

use crate::resolver::defs::resolve_definition;

#[derive(Debug)]
pub struct Asn1Compiler {
    // Modules belonging to this 'invocation' of compiler. Modules are maintined inside a `RefCell`
    // because we need Interior Mutability with the modules (for example to 'resolve' definitions
    // within the modules.
    modules: HashMap<String, RefCell<Asn1Module>>,

    // Resolved definitions: Definitions that we know about and are resolved into built-in or
    // Constructed types made up entirely of built-in types. We'll be using "Type" definitions from
    // this Map for code generation eventually.
    resolved_defs: HashMap<String, Asn1ResolvedDefinition>,

    // parameterized_defs are never 'resolved' In fact a given definition can be 'resolved' using
    // information from the Parameterized Definition. Note: Right now only `ParameterizedType` is
    // supported, but it should not change a whole lot if we support other `Parameterized*`.
    parameterized_defs: HashMap<String, Asn1Definition>,

    // object_classes similarly are never `resolved`. Information from the object classes can be
    // used to 'create' objects, which in turn can be used to create `Types`/`Values` (Usually
    // Constructed Sequence types, using CLASSREF.
    object_classes: HashMap<String, Asn1ObjectClassAssignment>,
}

impl Asn1Compiler {
    pub fn new() -> Self {
        Asn1Compiler {
            modules: HashMap::new(),
            resolved_defs: HashMap::new(),
            parameterized_defs: HashMap::new(),
            object_classes: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, module: Asn1Module) -> bool {
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

    fn sorted_modules(&self) -> Vec<String> {
        let mut ts = TopologicalSort::<String>::new();

        for cell in self.modules.values() {
            let module = cell.borrow();
            let imports = &module.imports;
            for m in imports.values() {
                ts.add_dependency(m.name.clone(), module.name.name.clone());
            }
            ts.insert(module.name.name.clone());
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

    pub fn resolve_definitions(&mut self) -> Result<(), Error> {
        let module_names = self.sorted_modules();
        for name in module_names {
            let mut module = self.modules.get(&name).unwrap().borrow_mut();
            //let module_definitions = module.definitions_sorted();
            for k in module.definitions_sorted() {
                eprintln!("k: {:#?}", k);
                let parsed_def = module.definitions.get_mut(&k).unwrap();
                if parsed_def.params.is_some() {
                    self.parameterized_defs
                        .insert(k.to_string(), parsed_def.clone());
                } else if let Asn1AssignmentKind::Class(ref c) = parsed_def.kind {
                    self.object_classes.insert(k.to_string(), c.clone());
                } else {
                    let resolved_def = resolve_definition(
                        parsed_def,
                        &self.resolved_defs,
                        &self.parameterized_defs,
                        &self.object_classes,
                    )?;
                    self.resolved_defs.insert(k.clone(), resolved_def);
                }
                parsed_def.resolved = true;
            }
        }
        eprintln!("Resolved: {:#?}", self.resolved_defs.keys());
        eprintln!("Parameterized: {:#?}", self.parameterized_defs.keys());
        eprintln!("Object Classes: {:#?}", self.object_classes.keys());
        Ok(())
    }
}
