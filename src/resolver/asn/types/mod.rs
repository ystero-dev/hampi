pub(crate) mod base;
pub(crate) mod constraints;
pub(crate) mod constructed;
pub(crate) mod ioc;

mod int;
pub(crate) use int::resolve_type;
