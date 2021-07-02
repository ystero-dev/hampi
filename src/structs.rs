#![allow(dead_code)]
//! A collection of structs used in [Hampi][`crate`] for ASN.1 compilation.
use std::collections::HashMap;

use super::base_types::*;

#[derive(Debug)]
pub enum Asn1ModuleTag {
    Explicit,
    Implicit,
    Automatic,
}

impl Default for Asn1ModuleTag {
    fn default() -> Self {
        Self::Explicit
    }
}

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

#[derive(Clone)]
pub struct OIDComponent {
    name: Option<String>,
    number: u32,
}

impl OIDComponent {
    pub fn new(name: Option<String>, number: u32) -> Self {
        Self { name, number }
    }
}

impl std::fmt::Display for OIDComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name.is_some() {
            write!(f, "{}({})", self.name.as_ref().unwrap(), self.number)
        } else {
            write!(f, "{}", self.number)
        }
    }
}

impl std::fmt::Debug for OIDComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
#[derive(Default, Clone)]
pub struct ObjectIdentifier {
    components: Vec<OIDComponent>,
}

impl ObjectIdentifier {
    pub fn new(components: Vec<OIDComponent>) -> Self {
        Self { components }
    }

    pub fn len(self) -> usize {
        self.components.len()
    }
}

impl std::fmt::Display for ObjectIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((first, rest)) = self.components.split_first() {
            write!(f, "{}", first)?;
            for c in rest {
                write!(f, ".{}", c)?;
            }
        } else {
            write!(f, "EMPTY")?;
        }
        write!(f, "")
    }
}

impl std::fmt::Debug for ObjectIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

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

#[derive(Debug, Default, Clone)]
pub struct Asn1ModuleName {
    name: String,
    oid: Option<ObjectIdentifier>,
}

impl Asn1ModuleName {
    pub fn new(name: String, oid: Option<ObjectIdentifier>) -> Self {
        Self { name, oid }
    }
}

/// Definition of a 'parsed' ASN Module
///
/// When an ASN module is successfully parsed, it contains a set of definitions that result from
/// parsing the assignments in a given module (Definitions within 'BEGIN' and 'END' tags.
/// Optionally, the module may have some definitions, that are imported from other modules and the
/// module may even 'export' some defintions. A module is uniquely identified by a name and object
/// identifier. In addition a module may support 'tagging' internal sequence values differently, so
/// information about it is kept as well.
#[derive(Debug, Default)]
pub struct Asn1Module {
    imports: HashMap<String, Asn1ModuleName>,
    exports: Option<Vec<Asn1Definition>>,
    name: Asn1ModuleName,
    tags: Asn1ModuleTag,
    types: HashMap<String, Asn1Definition>,
    values: HashMap<String, Asn1Definition>,
    exports_all: bool,
}

impl Asn1Module {
    pub fn name(mut self, name: Asn1ModuleName) -> Self {
        self.name = name;
        self
    }

    pub fn tags(mut self, tags: Asn1ModuleTag) -> Self {
        self.tags = tags;
        self
    }

    pub fn imports(mut self, imports: HashMap<String, Asn1ModuleName>) -> Self {
        self.imports = imports;
        self
    }
}
