# Hampi - ASN.1 compiler in Rust

The Goal of this project is to implement an ASN.1 Compiler in Rust which can generate Rust bindings for different ASN.1 specifications, which can be used in those protocol implementations in Rust.

Currently the focus is on working with 3GPP protocols for 5G, more specifically Core Network protocols like NGAP.

## Status

Right now specifications in the `specs/` directory can be parsed correctly(hopefully!) into an AST for the ASN.1 module definitions. This is in active development there are no public APIs at this time.

## Getting Started

Right now only a few test cases can be run through `cargo test`. Or one can checkout the AST for the protocols using `cargo run specs/ngap/*asn`.
