#![allow(dead_code)]
//! Structures for the 'Resolved' Types

pub(crate) mod defs;

pub(crate) mod types;

pub(crate) mod values;

mod int;
pub(crate) use int::Resolver;
