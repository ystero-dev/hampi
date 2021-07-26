//! Submodule for ASN.1 Specific handling of the Parser

mod module;
pub(super) use module::parse_module;

mod defs;

mod types;

mod values;

mod oid;

pub(crate) mod structs;
