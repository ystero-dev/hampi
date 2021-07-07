//! A simple utility to tokenize ASN files.

use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    for arg in &args[1..] {
        eprintln!("File: {}", arg);
        let file = File::open(arg)?;
        let mut tokens = hampi::tokenizer::tokenize(file)?;
        let module = hampi::parser::parse(&mut tokens)?;
        println!("{:#?}", module);
    }
    Ok(())
}
