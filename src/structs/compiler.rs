#![allow(dead_code)]
//! Structs related to ASN.1 Compiler
use std::cell::RefCell;
use std::collections::HashMap;

use topological_sort::TopologicalSort;

use crate::error::Error;

use crate::structs::parser::module::Asn1Module;
use crate::structs::resolver::defs::Asn1ResolvedDefinition;

use crate::resolver::defs::resolve_definition;

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
        eprintln!("sorted_modules: {:#?}", module_names);
        for name in module_names {
            let mut module = self.modules.get(&name).unwrap().borrow_mut();
            let module_definitions = module.definitions_mut();
            for (k, parsed_def) in module_definitions.iter_mut() {
                let resolved_def = resolve_definition(parsed_def, &self.all_definitions)?;
                self.all_definitions.insert(k.clone(), resolved_def);
                parsed_def.resolved = true;
            }
        }
        eprintln!("Resolved: {:#?}", self.all_definitions);
        Ok(())
    }
}
