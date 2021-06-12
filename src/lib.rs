//! ASN.1 Compiler in Rust
//!
//! Goal of the project is to develop a compiler for ASN.1 specifications primarily for
//! applicatoins in working with 3GPP standards. The idea is to be able to generate Rust (and
//! possibly other language(s) bindings from ASN.1 Specifications.

mod base_types;
pub mod error;
mod parser;
mod structs;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
