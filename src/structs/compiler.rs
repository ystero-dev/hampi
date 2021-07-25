#![allow(dead_code)]
//! Structs related to ASN.1 Compiler
use std::collections::HashMap;

use topological_sort::TopologicalSort;

use crate::error::Error;

use crate::structs::parser::module::Asn1Module;
use crate::structs::resolver::Resolver;

#[derive(Debug)]
pub struct Asn1Compiler {
    // Modules belonging to this 'invocation' of compiler. Modules are maintined inside a `RefCell`
    // because we need Interior Mutability with the modules (for example to 'resolve' definitions
    // within the modules.
    modules: HashMap<String, Asn1Module>,

    resolver: Resolver,
}

impl Asn1Compiler {
    pub fn new() -> Self {
        Asn1Compiler {
            modules: HashMap::new(),
            resolver: Resolver::new(),
        }
    }

    pub fn add_module(&mut self, module: Asn1Module) -> bool {
        let old = self.modules.insert(module.name.name.clone(), module);
        !old.is_none()
    }

    pub fn resolve_imports(&self) -> Result<bool, Error> {
        for (_, module) in self.modules.iter() {
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

        for module in self.modules.values() {
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
            let mut module = self.modules.get_mut(&name).unwrap();

            //let module_definitions = module.definitions_sorted();
            self.resolver.resolve_definitions(&mut module)?;
        }
        eprintln!(
            "Resolved Definitions: {:#?}",
            self.resolver.resolved_defs.keys()
        );
        eprintln!(
            "Parameterized Types: {:#?}",
            self.resolver.parameterized_defs.keys()
        );
        eprintln!("Object Classes: {:#?}", self.resolver.classes.keys());
        Ok(())
    }
}
