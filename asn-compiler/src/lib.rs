#![warn(missing_docs)]
//! ASN.1 Compiler in Rust
//!
//! Goal of the project is to develop a compiler for ASN.1 specifications primarily for
//! applicatoins in working with 3GPP standards. The idea is to be able to generate Rust (and
//! possibly other language(s)) bindings from ASN.1 Specifications. Initial support is targetted
//! for generating Rust bindings.

//! Error type for different types of Compilation Errors.
#[macro_use]
pub mod error;

/// ASN.1 Tokenizer and Related Types
#[macro_use]
pub mod tokenizer;

/// ASN.1 Parser and Related Types
pub mod parser;

/// ASN.1 Compiler Wrapper implmentation.
mod compiler;
pub use compiler::Asn1Compiler;

/// Types and Constraints resolution from the parsed types.
pub mod resolver;

/// Code Generation from the resolved types.
pub mod generator;
