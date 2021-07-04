//! A simple utility to tokenize ASN files.

use std::fs::File;
use std::io;

use hampi::parser;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    for arg in &args[1..] {
        eprintln!("File: {}", arg);
        let file = File::open(arg)?;
        let mut tokens = parser::tokenize(file)?;
        eprintln!("{:#?}", tokens);
        let module = parser::parse(&mut tokens)?;
        println!("{:#?}", module);
    }
    Ok(())
}
