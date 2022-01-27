//! Resolver Struct and it's implementation

use std::collections::{BTreeMap, HashMap};

use crate::error::Error;

use crate::parser::asn::structs::{defs::Asn1Definition, module::Asn1Module};

use crate::resolver::asn::structs::{defs::Asn1ResolvedDefinition, types::Asn1ResolvedType};

use crate::resolver::asn::defs::resolve_definition;

#[derive(Debug, Clone)]
pub(crate) struct Resolver {
    // Resolved definitions
    pub(crate) resolved_defs: BTreeMap<String, Asn1ResolvedDefinition>,

    // Unresolved Parameterized Definitions used by individual 'Type's to resolve themselves.
    pub(crate) parameterized_defs: HashMap<String, Asn1Definition>,

    // Object Classes: Used by Objects and Object Sets to resolves themselves.
    pub(crate) classes: HashMap<String, Asn1Definition>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            resolved_defs: BTreeMap::new(),
            parameterized_defs: HashMap::new(),
            classes: HashMap::new(),
        }
    }

    // Resolve Definitions: Algorithm
    //
    // First we need to resolve classes in the current module, because we need to resolve Object
    // and ObjectSets before we can process definitions. The Objects and ObjectSets need to be
    // handled before other definitions because Objects may refer to some Types that may be
    // required later on while resolving some other Types and if those 'Type's remain hidden within
    // Objects, we can always end up having some missing definition(s).
    //
    // These first two steps are actually part of parsing, which can only be done when all Class
    // definitions are avaiable (Think of it as macro invocation in Rust, which is before the Type
    // resolution, but after an individual file is parsed!).
    //
    // After that we resolve definitions in a Topologically sorted order. Fairly straight forward.
    // We do not need to do any `Pending` definitions, as we were doing before.
    pub(crate) fn resolve_definitions(&mut self, module: &mut Asn1Module) -> Result<(), Error> {
        // We need to first get Classes in the current module - resolved
        self.resolve_classes_in_current_module(module);

        module.resolve_object_classes(&self.classes)?;

        let definitions_sorted = module.definitions_sorted();
        for k in definitions_sorted {
            let parsed_def = module.get_definition_mut(&k);
            if parsed_def.is_none() {
                eprintln!(
                    "Definition '{}' Not found! It's Okay for certain Dummy References",
                    k
                );
                continue;
            }
            let parsed_def = parsed_def.unwrap();
            if parsed_def.params.is_some() {
                self.parameterized_defs
                    .insert(k.to_string(), parsed_def.clone());
                parsed_def.resolved = true;
            } else if parsed_def.is_class_assignment() {
                self.classes.insert(k.to_string(), parsed_def.clone());
                parsed_def.resolved = true;
            } else {
                let resolved_def = resolve_definition(parsed_def, self)?;
                self.resolved_defs.insert(k.clone(), resolved_def);
            }
            parsed_def.resolved = true;
        }

        for k in module.definitions_sorted() {
            let parsed_def = module.get_definition_mut(&k);
            if parsed_def.is_none() {
                eprintln!(
                    "Definition '{}' Not found! It's Okay for certain Dummy References",
                    k
                );
                continue;
            }
            let parsed_def = parsed_def.unwrap();
            if !parsed_def.resolved {
                println!(
                    "UNRESOLVED: Definition: {} in module : {} not resolved!",
                    k,
                    module.get_module_name()
                );
            }
        }

        Ok(())
    }

    pub(crate) fn get_resolved_types(&self) -> Vec<(&String, &Asn1ResolvedType)> {
        self.resolved_defs
            .iter()
            .filter_map(|(k, v)| match v {
                Asn1ResolvedDefinition::Type(ref t) => Some((k, t)),
                _ => None,
            })
            .collect::<Vec<(&String, &Asn1ResolvedType)>>()
    }

    fn resolve_classes_in_current_module(&mut self, module: &Asn1Module) {
        for (k, def) in module.get_definitions() {
            if def.is_class_assignment() {
                self.classes.insert(k.clone(), def.clone());
            }
        }
    }
}
