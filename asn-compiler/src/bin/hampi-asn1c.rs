//! A simple utility to tokenize ASN files.

use std::fs::File;
use std::io;

use clap::Parser;

use asn1_compiler::{
    generator::{Codec, Derive, Visibility},
    Asn1Compiler,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(last = true)]
    files: Vec<String>,

    /// Name of the Rust Module to write generated code to.
    #[arg(short, long)]
    module: String,

    #[arg(short)]
    debug: bool,

    /// Visibility of Generated Structures and members:
    #[arg(long, value_enum, default_value_t=Visibility::Public)]
    visibility: Visibility,

    /// ASN.1 Codecs to be Supported during code generation.
    /// Specify multiple times for multiple codecs. (eg. --codec aper --codec uper)
    #[arg(long, required = true)]
    codec: Vec<Codec>,

    /// Generate code for these derive macros during code generation.
    #[arg(long)]
    derive: Vec<Derive>,
}

fn main() -> io::Result<()> {
    let mut cli = Cli::parse();

    if !cli.derive.contains(&Derive::Debug) {
        cli.derive.push(Derive::Debug);
    }

    let mut compiler = Asn1Compiler::new(
        &cli.module,
        cli.debug,
        &cli.visibility,
        cli.codec.clone(),
        cli.derive.clone(),
    );

    if cli.files.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No Input files Specified",
        ));
    }

    for file in cli.files {
        let file = File::open(file)?;
        let mut tokens = asn1_compiler::tokenizer::tokenize(file)?;
        let mut modules = asn1_compiler::parser::parse(&mut tokens)?;

        loop {
            let module = modules.pop();
            if module.is_none() {
                break;
            }
            let module = module.unwrap();
            compiler.add_module(module);
        }
    }
    compiler.resolve_modules()?;

    compiler.generate()?;

    Ok(())
}
