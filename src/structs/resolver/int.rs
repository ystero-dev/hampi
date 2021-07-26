//! Resolver Struct and it's implementation

use std::collections::HashMap;

use crate::error::Error;

use crate::parser::asn::structs::{defs::Asn1Definition, module::Asn1Module};

use crate::structs::resolver::defs::Asn1ResolvedDefinition;

use crate::resolver::defs::resolve_definition;

#[derive(Debug, Clone)]
pub(crate) struct Resolver {
    // Resolved definitions
    pub(crate) resolved_defs: HashMap<String, Asn1ResolvedDefinition>,

    // Unresolved Parameterized Definitions used by individual 'Type's to resolve themselves.
    pub(crate) parameterized_defs: HashMap<String, Asn1Definition>,

    // Object Classes: Used by Objects and Object Sets to resolves themselves.
    pub(crate) classes: HashMap<String, Asn1Definition>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            resolved_defs: HashMap::new(),
            parameterized_defs: HashMap::new(),
            classes: HashMap::new(),
        }
    }

    pub(crate) fn resolve_definitions(&mut self, module: &mut Asn1Module) -> Result<(), Error> {
        for k in module.definitions_sorted() {
            let parsed_def = module.get_definition_mut(&k).unwrap();
            if parsed_def.params.is_some() {
                self.parameterized_defs
                    .insert(k.to_string(), parsed_def.clone());
            } else if parsed_def.is_class_assignment() {
                self.classes.insert(k.to_string(), parsed_def.clone());
            } else {
                let resolved_def = resolve_definition(parsed_def, self)?;
                self.resolved_defs.insert(k.clone(), resolved_def);
            }
            parsed_def.resolved = true;
        }
        Ok(())
    }
}
