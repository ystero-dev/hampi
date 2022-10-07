//! A simple utility to tokenize ASN files.

use std::fs::File;
use std::io;

use clap::Parser;

use asn1_compiler::{generator::Visibility, Asn1Compiler};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    files: Vec<String>,

    /// Name of the Rust Module to write generated code to.
    #[arg(short, long)]
    module: String,

    #[arg(short)]
    debug: bool,

    /// Visibility of Generated Structures and members:
    #[arg(long, value_enum, default_value_t=Visibility::Public)]
    visibility: Visibility,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut compiler = Asn1Compiler::new(&cli.module, cli.debug, &cli.visibility);

    if cli.files.len() == 0 {
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
