# Hampi - Rust ASN.1 Toolkit

[![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/gabhijit/hampi/build%20on%20pull%20request/master)](https://github.com/gabhijit/hampi/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/asn1-codecs?label=asn1-codecs)](https://crates.io/crates/asn1-codecs)
[![Crates.io](https://img.shields.io/crates/v/asn1-compiler?label=asn1-compiler)](https://crates.io/crates/asn1-compiler)
[![Crates.io](https://img.shields.io/crates/v/asn1_codecs_derive?label=asn1_codecs_derive)](https://crates.io/crates/asn1_codecs_derive)
The Goal of this project is to implement an ASN.1 Compiler in Rust which can generate Rust bindings for different ASN.1 specifications, which can be used in various protocol implementations in Rust. Also, support for different ASN.1 encoding rules like DER, BER, PER is planned to be implemented as ASN.1 codecs.

While it is certainly desirable to have a fully compliant ASN.1 Compiler, initial focus of the project is to be able to generate code that can be used in other software. First goal is to work with 3GPP specifications for protocols like Radio Network Access Protocl (RANAP), S1 Application protocl (S1AP) and NG Application Protocol (NGAP). Consequently current implementation also focuses on codecs used by these specifications viz. Aligned Packed Encoding Rules (APER).

The use cases targeted by this implementation are similar to the use cases targeted by `asn1c` compiler, which generates `C` bindings from the ASN.1 Specifications.

This project is divided into three crates -

1. asn1-compiler [![docs.rs](https://img.shields.io/docsrs/asn1-compiler?label=asn1-compiler)](https://docs.rs/asn1-compiler/latest/asn1_compiler/): This crate provides the actual ASN.1 Compiler. Typically a utility (`hampi-asn1c`) will generate Rust structures starting with ASN.1 Specifications. Basic working features required to work with the 3GPP specifications is provided by the compiler, this includes Parameterized Types, Information Object Classes and Type Constraints.

2. asn1-codecs [![docs.rs](https://img.shields.io/docsrs/asn1-codecs?label=asn1-codecs)](https://docs.rs/asn1-codecs/latest/asn1_codecs/): Support for different encodings supported by individual ASN.1 specifications is provided in this crate. Currently only APER Codec is supported. Each Codec is supported as a `trait` implementing `encode` and `decode` functions. Support for different 'encoding rules' will be implemented in this crate and then the derive macros will utilize this code to actually generate the encoding support for Rust Structures generated by the compiler above.

3. asn1_codecs_derive [![docs.rs](https://img.shields.io/docsrs/asn1_codecs_derive?label=asn1_codecs_derive)](https://docs.rs/asn1_codecs_derive/latest/asn1_codecs_derive/): This crate provides the `derive` macros for the codecs in `asn-codecs`. The code generated using `asn-compiler` can be directed to `derive` appropriate codecs by passing the flags during compilation.


## Supported Codecs

- APER

## Getting Started

### `build.rs` Support

Typically the compiler can be invoked also using `build.rs` mechanism. An example `build.rs` is provided in the `examples/` sub project. And the code generated through this `build.rs` can be integrated into your project. Examples of that is provided in `examples/tests/` directory.

### Using CLI tool

A tool `hampi-rs-asn1c` can be installed using `cargo install hampi-rs-asn1c` and then following the CLI usage.

### Running Test Cases

1. Test cases can be run through `cargo test`.

### Generating ASN.1 spec files

Currently there are two ways of generating the 'ASN.1' spec files from the '.docx' specifications.

1. Using the script inside `examples/specs/parse_specs.py`. This is recommended way. (note: this requires installation of `docx` or `python-docx` in the case of Python3).
2. Using the rust binary `extract-asn-spec`. Note: this is not the recommended way and your mileage may vary. Eventual goal is to make this the default way - so as to also have a `build.rs` integration. But this is for now a work in progress.

