#![allow(dead_code)]
//! Structs related to ASN.1 Compiler
use std::collections::HashMap;

use topological_sort::TopologicalSort;

use crate::error::Error;

use crate::parser::asn::structs::module::Asn1Module;

use crate::resolver::Resolver;

/// ASN.1 Compiler Struct.
///
/// An application should create a Compiler Structure and will call Public API functions on the
/// compiler.
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
        let old = self.modules.insert(module.get_module_name(), module);
        !old.is_none()
    }

    /// Resolve Modules order and definitions within those modules.
    ///
    /// First Makes sure that all the modules that have IMPORTs are indeed added to us. Then
    /// definitions in each of the modules are 'resolved'. Calls the `Resolver` functions to do
    /// that. Modules are Topologically Sorted before they are resolved and definitions within
    /// modules are topologically sorted as well. This makes Error handling for undefined
    /// definitions much easier.
    // FIXME: Support the case where module is imported by a name different from it's actual name.
    pub fn resolve_modules(&mut self) -> Result<(), Error> {
        self.resolve_imports()?;
        self.resolve_definitions()
    }

    fn resolve_imports(&self) -> Result<(), Error> {
        for (_, module) in self.modules.iter() {
            for (import, module_name) in module.get_imported_defs() {
                let target = self.modules.get(module_name.name_as_str());
                if target.is_none() {
                    return Err(resolve_error!(
                        "Module '{}', corresponding to definition '{}' not found!",
                        module_name.name_as_str(),
                        import
                    ));
                }
            }
        }
        eprintln!("All IMPORTS in All Modules Resolved!");
        Ok(())
    }

    fn sorted_modules(&self) -> Vec<String> {
        let mut ts = TopologicalSort::<String>::new();

        for module in self.modules.values() {
            let imports = module.get_imported_defs();
            for (_, m) in imports {
                ts.add_dependency(m.name(), module.get_module_name())
            }
            ts.insert(module.get_module_name());
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

    fn resolve_definitions(&mut self) -> Result<(), Error> {
        let module_names = self.sorted_modules();
        for name in module_names {
            let mut module = self.modules.get_mut(&name).unwrap();

            //let module_definitions = module.definitions_sorted();
            self.resolver.resolve_definitions(&mut module)?;
        }
        eprintln!("Resolved Definitions: {:#?}", self.resolver.resolved_defs);
        eprintln!(
            "Parameterized Types: {:#?}",
            self.resolver.parameterized_defs.keys()
        );
        eprintln!("Object Classes: {:#?}", self.resolver.classes);
        Ok(())
    }
}
