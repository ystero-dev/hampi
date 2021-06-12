//! A collection of structs used in [Hampi][crate] for ASN.1 compilation.

use super::base_types::*;

struct LineColumn {
    line: usize,
    column: usize,
}

struct Span {
    start: LineColumn,
    end: LineColumn,
}

struct Token;

struct ObjectIdentifier;

struct Asn1TagType;

struct BaseTypeValue;

enum Asn1BaseType {
    Integer(IntegerType),
    Enumerated(EnumeratedType),
    Boolean(BooleanType),
    Null(NullType),
}

struct BaseType {
    base: Asn1BaseType,
    constraints: Asn1Constraint,
    identifier: String,
}

enum ResolvedType {
    TypeRef(Box<ResolvedType>),
    Base(BaseType),
}

struct TypeDefinition {}

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
pub struct ValueDefinition {
    identifier: String,
    r#type: ResolvedType,
    value: Option<BaseTypeValue>,
    name: Option<String>,
    resolved: bool,
}

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
pub struct Asn1Module {
    imports: Option<Vec<Asn1Definition>>,
    exports: Option<Vec<Asn1Definition>>,
    name: String,
    oid: Option<ObjectIdentifier>,
    tags: Asn1TagType,
    definitions: Vec<Asn1Definition>,
}
