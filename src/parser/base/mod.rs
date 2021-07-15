//! Parsing for Base types

mod utils;

mod integer;
pub(crate) use integer::parse_integer_type;

mod enumerated;
pub(crate) use enumerated::parse_enumerated_type;
