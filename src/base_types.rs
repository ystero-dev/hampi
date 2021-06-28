#![allow(dead_code)]
//! ASN.1 Base Types

pub struct IntegerType {
    inner: i64,
}

pub struct BooleanType {
    inner: bool,
}

pub struct NullType {
    inner: (),
}

#[derive(Debug)]
pub struct Asn1Constraint;

enum EnumVariant {
    Ellipsis,
    Named(String, i64),
}

pub struct EnumeratedType {
    variants: Vec<EnumVariant>,
    ext_marker: usize,
}
