# Architecture of ASN.1 Compiler

The purpose of this file is to help anyone interested in taking a closer look at the code with the intention of contributing to provide lay of the land. If one is looking at the code for the first time, it's perhaps a good idea to go through this file quickly (and suggest any improvements if required after reading this file and some code. :-)).

Any compiler will have the following stages - tokenizing, parsing, symbol resolution and code generation. Similar architecture is followed in implementation of this crate.

## `tokenizer`

This directory contains code for tokenizing an input file into `Token`s that are then used by the `parser` to generate the parse tree. Generally the reader of the code is hardly required to change this part of the code because _lexemes_ for the language are already covered. This module provides a single public API `tokenize`.


## `parser`
`
This directory contains code for the parser and a very high level API `parse`. In addition, data structures generated as a result of parsing _viz_ our representation of parsed ASN.1 syntax are defined and `impl`ed here. For different ASN.1 definitions, types etc. the internal representation structures are defined in `parser/asn/structs/` etc. A general convention we follow is - the types are defined inside `structs` and actual implementation mostly parsing functions are inside `parser/asn/`.  Code for different ASN.1 specific items (eg. definitions, types, constraints, are within their own module.) This structure is followed for other stages as well. For instance, each ASN.1 file represents an ASN.1 module and a module consists of some header (imported definitions, exported definitions module Object Identifier etc.) and Definitions within the modules. Each definition will be of a different Kind - eg. it will be a "Type" or a "Value" definition and based upon inferred type of the definition (from the grammar), further parsing happens within a specific module. If this is too much for now, one can start from `parser/module.rs` and follow `parser/defs.rs` and then from there.

One thing to note here is the structures generated here will be used by the next stage of the compilers - thus all the "Parsed" types will be used by the next stage of "Symbol Resolution"

## `resolver`

This module is mainly concerned with 'resolving' references across modules, and across definitions within the modules. The code structure inside the `resolver` module is similar to `parser` and same approach used in following the code in `parser` can be followed here. There are a couple of things worth noting, all the 'definitions' within ASN.1 module(s) are sorted using topological sort so that the definitions 'depending' upon others are sorted later. Similar to parser this stage results in generation of a lot of 'resolved' data structures (see `resolver/asn/structs/`) are generated. These data structures are used by the next stage that is code generation. For ASN.1 information object classes, the resolution of 'Objects' and 'Object Sets' happens in this stage.

## `generator`

Once the definitions within modules are represented in their 'resolved' state, those structures can then be used to generate the actual 'Rust' code. The internal structure of the 'generator' module is also very similar to the above two modules and Rust 'structures' are generated from the Resolved ASN.1 Types here. Every ASN.1 type is represented as either a `struct` or an `enum` in Rust (`enum` in case of `CHOICE` ASN.1 type and Object Sets for a `SEQUENCE` type.). Type constraints (that are also) resolved in the previous step) are then used to generate the 'attributes' for the `struct` or `enum`. These attributes are then used by the 'codecs' during seriaization.

## `compiler`

This is more of a driver for all the stages along with taking care of calling the individual steps with appropriate configuration.


