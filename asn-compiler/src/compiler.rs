#![allow(dead_code)]
//! Structs related to ASN.1 Compiler

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

use topological_sort::TopologicalSort;

use crate::error::Error;

use crate::parser::asn::structs::module::Asn1Module;

use crate::generator::{Generator, Visibility};
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

    // Holds the 'Resolver' that is used for 'resolv'ing definitions.
    resolver: Resolver,

    // Holds the 'Generator' that is used for 'generate'ing the code for the 'resolved types'.
    generator: Generator,

    // Holds the file name for the output module.
    output_filename: String,

    // Debug Print during Code generation
    debug: bool,
}

impl Default for Asn1Compiler {
    fn default() -> Self {
        Asn1Compiler::new("default.rs", false, &Visibility::Public)
    }
}

impl Asn1Compiler {
    /// Create a new Instance of the Compiler structure.
    pub fn new(output: &str, debug: bool, visibility: &Visibility) -> Self {
        Asn1Compiler {
            modules: HashMap::new(),
            resolver: Resolver::new(),
            generator: Generator::new(visibility), // FIXME: Hard coded
            output_filename: output.to_string(),
            debug,
        }
    }

    /// Add a module to the list of known modules.
    ///
    /// If the module alredy exists, returns `false` else returns `true`.
    pub fn add_module(&mut self, module: Asn1Module) -> bool {
        self.modules
            .insert(module.get_module_name(), module)
            .is_some()
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

    /// Generate the code
    pub fn generate(&mut self) -> Result<(), Error> {
        let input_text = self.generator.generate(&self.resolver)?;
        let output_text = self.rustfmt_generated_code(&input_text)?;

        let mut output_file = File::create(&self.output_filename).map_err(|e| {
            let errorstr = format!("Error {} Creating File: {}", e, self.output_filename);
            Error::CodeGenerationError(errorstr)
        })?;

        output_file
            .write_all(output_text.as_bytes())
            .map_err(|e| Error::CodeGenerationError(e.to_string()))?;

        eprintln!("\n\nWrote generated code to '{}'.", self.output_filename);

        Ok(())
    }

    fn rustfmt_generated_code(&self, code: &str) -> Result<String, Error> {
        let rustfmt_binary = "rustfmt"; // TODO: Get from `env` , 'custom path' etc.
        let mut cmd = Command::new(rustfmt_binary);

        cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| resolve_error!("{:#?}", e))?;
        let mut child_stdin = child.stdin.take().unwrap();
        let mut child_stdout = child.stdout.take().unwrap();

        let code = code.to_owned();
        let stdin_handle =
            ::std::thread::spawn(move || match child_stdin.write_all(code.as_bytes()) {
                Ok(_) => code,
                Err(_) => "write error in rustfmt".to_owned(),
            });

        let mut output = vec![];
        io::copy(&mut child_stdout, &mut output).map_err(|e| resolve_error!("{:#?}", e))?;

        let status = child.wait().map_err(|e| resolve_error!("{:#?}", e))?;

        match String::from_utf8(output) {
            Ok(formatted_output) => match status.code() {
                Some(0) => Ok(formatted_output.to_owned()),
                _ => Err(resolve_error!("`rustfmt` failed to write some bindings.")),
            },
            _ => Ok(stdin_handle.join().unwrap().to_owned()),
        }
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
        if self.debug {
            eprintln!("All IMPORTS in All Modules Resolved!");
        }
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
        if self.debug {
            eprintln!(
                "Resolved Definitions: {:#?}",
                self.resolver.resolved_defs.keys()
            );
            eprintln!(
                "Parameterized Types: {:#?}",
                self.resolver.parameterized_defs.keys()
            );
            eprintln!("Object Classes: {:#?}", self.resolver.classes.keys());
        }
        Ok(())
    }
}
