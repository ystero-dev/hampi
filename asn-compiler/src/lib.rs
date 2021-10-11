#![warn(missing_docs)]
//! ASN.1 Compiler in Rust
//!
//! Goal of the project is to develop a compiler for ASN.1 specifications primarily for
//! applicatoins in working with 3GPP standards. The idea is to be able to generate Rust (and
//! possibly other language(s)) bindings from ASN.1 Specifications.

#[macro_use]
pub mod error;

/// ASN.1 Tokenizer and Related Types
#[macro_use]
pub mod tokenizer;

/// ASN1. Parser and Related Types
pub mod parser;

mod compiler;
pub use compiler::Asn1Compiler;

pub mod resolver;

pub mod generator;