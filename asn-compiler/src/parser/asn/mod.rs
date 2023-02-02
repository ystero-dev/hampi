//! Submodule for ASN.1 Specific handling of the Parser

/// ASN.1 Modules (Single ASN.1 Files) handling functionality.
mod module;
pub(super) use module::parse_module;

/// ASN.1 Definitions (Type, Value, CLASS etc.)
mod defs;

/// Implementation of Parsing of ASN.1 Types.
pub(crate) mod types;

/// Implementation of Parsing of ASN.1 Values (`Type := Value`)
pub(crate) mod values;

/// Implementation of Object Identifier.
mod oid;
// Required during resolution of Object Identifier values.
pub(crate) use oid::{parse_object_identifier, WELL_KNOWN_OID_NAMES};

/// Output Types of the Parsers.
pub(crate) mod structs;
