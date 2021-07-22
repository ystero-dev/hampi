#![allow(dead_code)]
//! Structures for the 'Resolved' Types

pub mod base;

use base::ResolvedBaseType;

#[derive(Debug)]
pub(crate) enum Asn1ResolvedType {
    Base(ResolvedBaseType),
}

#[derive(Debug)]
pub(crate) enum Asn1ResolvedDefinition {
    Type(Asn1ResolvedType),
    Value(Asn1ResolvedValue),
}

/// An INTEGER value will be represented by a BaseInteger type when 'Resolved'.
pub(crate) type BaseInteger = i64;

/// An ENUMERATED Value will be represented by a BaseEnum type when 'Resolved'.
pub(crate) type BaseEnum = i64;

/// A BOOLEAN Value will be represented by a a BaseBoolean type when 'Resolved'.
pub(crate) type BaseBoolean = bool;

/// Any CharacterString value will be represented by a BaseCharString type when 'Resolved'.
pub(crate) type BaseCharString = bool;

/// Any OCTET STRING value will be represente by a BaseOctetString type when 'Resolved'.
pub(crate) type BaseOctetSTring = Vec<u8>;

#[derive(Debug)]
pub(crate) struct Asn1ResolvedIntegerValue {
    typeref: Asn1ResolvedType,
    value: BaseInteger,
}

#[derive(Debug)]
pub(crate) struct Asn1ResolvedEnumValue {
    typeref: Asn1ResolvedType,
    value: BaseEnum,
}

#[derive(Debug)]
pub(crate) enum Asn1ResolvedValue {
    Integer(Asn1ResolvedIntegerValue),
    Enum(Asn1ResolvedEnumValue),
}
