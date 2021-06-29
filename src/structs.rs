#![allow(dead_code)]
//! A collection of structs used in [Hampi][`crate`] for ASN.1 compilation.

use super::base_types::*;
use super::oid::ObjectIdentifier;

#[derive(Debug)]
pub struct Asn1TagType;

#[derive(Debug)]
struct BaseTypeValue;

#[derive(Debug)]
enum Asn1BaseType {
    Integer,
    Enumerated,
    Boolean,
    Null,
}

#[derive(Debug)]
pub struct BaseType {
    base: Asn1BaseType,
    constraints: Vec<Asn1Constraint>,
    identifier: String,
}

#[derive(Debug)]
pub enum ResolvedType {
    TypeRef(Box<ResolvedType>),
    Base(BaseType),
}

#[derive(Debug)]
pub struct TypeDefinition {}

/// A Value definition in ASN.1 Syntax
///
/// A Value definition in ASN.1 Syntax may look like following -
///
/// ```asn
///
///     maxnoofElements INTEGER ::= 1000
///
/// ```
///
/// A Value definition resolves to the identifier (This will be used as a key when looking up for
/// values in a given module. The 'resolved' type of the value. See [`ResolvedType`]. Actual value
/// (refering to a value of base ASN.1 Type. See [`base_types`]). Note: When the value is
/// 'resolved', the actual value contains one of the base type values after all constraints are
/// validated.
#[derive(Debug)]
pub struct ValueDefinition {
    identifier: String,
    r#type: ResolvedType,
    value: Option<BaseTypeValue>,
    name: Option<String>,
    resolved: bool,
}

#[derive(Debug)]
pub enum Asn1Definition {
    Value(ValueDefinition),
    Type(TypeDefinition),
    //ValueSetDefinition,
    //InfoObjectClassDefinition,
    //InfoObjectDefinition,
    //InfoObjectSetDefinition,
}

/// Definition of a 'parsed' ASN Module
///
/// When an ASN module is successfully parsed, it contains a set of definitions that result from
/// parsing the assignments in a given module (Definitions within 'BEGIN' and 'END' tags.
/// Optionally, the module may have some definitions, that are imported from other modules and the
/// module may even 'export' some defintions. A module is uniquely identified by a name and object
/// identifier. In addition a module may support 'tagging' internal sequence values differently, so
/// information about it is kept as well.
#[derive(Debug)]
pub struct Asn1Module {
    imports: Option<Vec<Asn1Definition>>,
    exports: Option<Vec<Asn1Definition>>,
    name: String,
    oid: Option<ObjectIdentifier>,
    tags: Asn1TagType,
    definitions: Vec<Asn1Definition>,
}

impl Asn1Module {
    pub fn empty(name: &str) -> Self {
        Asn1Module {
            imports: None,
            exports: None,
            name: name.to_string(),
            oid: None,
            tags: Asn1TagType {},
            definitions: vec![],
        }
    }

    pub fn oid(mut self, oid: Option<ObjectIdentifier>) -> Self {
        self.oid = oid;
        self
    }
}
