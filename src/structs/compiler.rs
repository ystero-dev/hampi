//! Structs related to ASN.1 Compiler
use std::collections::HashMap;

use crate::error::Error;
use crate::structs::module::Asn1Module;

#[derive(Debug)]
pub struct Asn1Compiler {
    /// Modules belonging to this 'invocation' of compiler.
    modules: HashMap<String, Asn1Module>,
}

impl Asn1Compiler {
    pub fn new() -> Self {
        Asn1Compiler {
            modules: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, module: Asn1Module) -> bool {
        eprintln!("Adding Module!");
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
}
