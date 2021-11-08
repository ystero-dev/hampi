//! A simple utility to tokenize ASN files.

use std::fs::File;
use std::io;

use asn1_compiler::Asn1Compiler;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let mut compiler = Asn1Compiler::new();

    for arg in &args[1..] {
        eprintln!("File: {}", arg);
        let file = File::open(arg)?;
        let mut tokens = asn1_compiler::tokenizer::tokenize(file)?;
        eprintln!("tokens: {}", tokens.len());
        let mut modules = asn1_compiler::parser::parse(&mut tokens)?;

        loop {
            let module = modules.pop();
            if module.is_none() {
                break;
            }
            let module = module.unwrap();
            //eprintln!("module: {:#?}", module);
            compiler.add_module(module);
        }
    }
    compiler.resolve_modules()?;

    compiler.generate()?;

    Ok(())
}
