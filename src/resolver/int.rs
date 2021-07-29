//! Resolver Struct and it's implementation

use std::collections::HashMap;

use topological_sort::TopologicalSort;

use crate::error::Error;

use crate::parser::asn::structs::{defs::Asn1Definition, module::Asn1Module};

use crate::resolver::asn::structs::defs::Asn1ResolvedDefinition;

use crate::resolver::asn::defs::resolve_definition;

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

    // Resolve Definitions: Algorithm
    //
    // Since this whole thing is a bit complicated, it's a good idea to write down the algorithm
    // here about how we are resolving the definitions.
    //
    // First we obtain a set of all definitions in a module Topologically Sorted. Thus when we
    // visit a definition, any dependencies are already resolved. However this is not still
    // sufficient, since the Objects and ObjectSets can have dependent Type or Value that can only
    // be known while processing the Object. The `Type` dependencies are thus not figured out
    // already. Hence we get Reference Error. For this reason, we simply postpone Object and Object
    // Set resolution, after all other definitions are sorted. These definitions are called
    // 'Pending' definitions.
    //
    // Once everything other than Objects and ObjectSets is processed, we process `Pending`
    // definitions. However, we need to make sure that these definitions are also sorted, ie. the
    // Objects an ObjectSet is referencing needs to be resolved before the ObjectSet is resolved.
    // Thus we 'Sort' these Pending Definitions again and finally process them.
    pub(crate) fn resolve_definitions(&mut self, module: &mut Asn1Module) -> Result<(), Error> {
        // We need to first get Classes in the current module - resolved
        self.resolve_classes_in_current_module(module);

        module.resolve_object_classes(&self.classes)?;

        let mut pending_definitions: HashMap<String, Asn1Definition> = HashMap::new();
        for k in module.definitions_sorted() {
            let parsed_def = module.get_definition_mut(&k).unwrap();
            if parsed_def.params.is_some() {
                self.parameterized_defs
                    .insert(k.to_string(), parsed_def.clone());
                parsed_def.resolved = true;
            } else if parsed_def.is_class_assignment() {
                self.classes.insert(k.to_string(), parsed_def.clone());
                parsed_def.resolved = true;
            } else {
                let resolved_def = resolve_definition(parsed_def, self, false)?;
                if let Asn1ResolvedDefinition::Pending(ref p) = resolved_def {
                    pending_definitions.insert(k.clone(), p.clone());
                } else {
                    self.resolved_defs.insert(k.clone(), resolved_def);
                    parsed_def.resolved = true;
                }
            }
        }

        // Now process pending definitions.
        let sorted_pending = Self::sorted_pending(&pending_definitions);
        for k in sorted_pending {
            let parsed_def = pending_definitions.get_mut(&k);
            if parsed_def.is_some() {
                let parsed_def = parsed_def.unwrap();
                let resolved_def = resolve_definition(&parsed_def, self, true)?;
                self.resolved_defs.insert(k.clone(), resolved_def);
                let module_definition = module.get_definition_mut(&k).unwrap();
                module_definition.resolved = true;
            }
        }

        for k in module.definitions_sorted() {
            let parsed_def = module.get_definition_mut(&k).unwrap();
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

    fn sorted_pending(pending_definitions: &HashMap<String, Asn1Definition>) -> Vec<String> {
        let mut ts = TopologicalSort::<String>::new();

        for (k, v) in pending_definitions.iter() {
            for r in v.dependent_references() {
                ts.add_dependency(r.clone(), k.clone());
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

    fn resolve_classes_in_current_module(&mut self, module: &Asn1Module) -> () {
        for (k, def) in module.get_definitions() {
            if def.is_class_assignment() {
                self.classes.insert(k.clone(), def.clone());
            }
        }
    }
}
