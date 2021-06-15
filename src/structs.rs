#![allow(dead_code)]
//! A collection of structs used in [Hampi][`crate`] for ASN.1 compilation.

use super::base_types::*;

/// Line and Column in the source where the token begins.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct LineColumn {
    line: usize,
    column: usize,
}

impl LineColumn {
    pub fn new(line: usize, column: usize) -> Self {
        LineColumn { line, column }
    }
}

/// Span of a Token in the source.
#[derive(Debug)]
pub struct Span {
    start: LineColumn,
    end: LineColumn,
}

impl Span {
    pub fn new(start: LineColumn, end: LineColumn) -> Self {
        Span { start, end }
    }
}

/// Token Types
///
/// Each parsed token should be of one of the following types.
#[derive(Debug)]
pub enum TokenType {
    CurlyBegin,        // "{"
    CurlyEnd,          // "}"
    RoundBegin,        // "("
    RoundEnd,          // ")"
    SquareBegin,       // "["
    SquareEnd,         // "]"
    SeqExtensionBegin, // "[["
    SeqExtensionEnd,   // "]]"
    Extension,         // "..."
    RangeSeparator,    // ".."
    Assignment,        // "::="
    Colon,             // ':'
    Identifier,        // Identifiers and all references.
    Keyword,           // eg. "INTEGER", "ENUMERATED", "RELATIVE-OID", "TYPE-IDENTIFIER"
    Comment,           // "-- and everything after up to newline or EOF
    AndIdentifier,     // "&Attribute-Type", "&id" etc.
    AtNotation,        // '@.'Identifier or '@'Identifier
    BitString,         // '010...'B
    HexString,         // 'FEEDBAC...'h
}

/// A parsed token before AST is created.
///
/// Going through an ASN.1 module source results in a vector of parsed tokens of appropriate types.
/// The parsed tokens are then used to 'resolve' type and value definitions to obtain the instances
/// of respective structures.
#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub span: Span,
    pub text: String,
}

struct ObjectIdentifier;

struct Asn1TagType;

struct BaseTypeValue;

enum Asn1BaseType {
    Integer,
    Enumerated,
    Boolean,
    Null,
}

pub struct BaseType {
    base: Asn1BaseType,
    constraints: Vec<Asn1Constraint>,
    identifier: String,
}

pub enum ResolvedType {
    TypeRef(Box<ResolvedType>),
    Base(BaseType),
}

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
