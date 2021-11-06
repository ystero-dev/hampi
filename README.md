# Hampi - ASN.1 compiler in Rust

The Goal of this project is to implement an ASN.1 Compiler in Rust which can generate Rust bindings for different ASN.1 specifications, which can be used in those protocol implementations in Rust.

This project is divided into three crates -

1. asn1-compiler: This crate provides the actual ASN.1 Compiler.
2. asn-codecs: Support for different encodings supported by individual ASN.1 specifications is provided in this crate. Currently only APER Codec is supported. Each Codec is supported as a `trait` implementing `encode` and `decode` functions.
3. asn-codecs-derive: This crate provides the `derive` macros for the codecs in `asn-codecs`. The code generated using `asn-compiler` can be directed to `derive` appropriate codecs by passing the flags during compilation.

## Status

Currently specifications from the `specs/` directory can be compiled into respective Rust modules. See below for how these Rust modules can be used.

## Getting Started

### Running Test Cases

1. Test cases can be run through `cargo test`.
2. Right now `examples/` directory contains simple `decode` test cases for modules generated using the `asn-compiler`. This support will be improved to include the generated modules using mechanisms like `build.rs`.



