#![allow(dead_code)]
//! A collection of structs used in [Hampi][`crate`] for ASN.1 compilation.

use super::base_types::*;

use super::token_types::TokenType;

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

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

/// Span of a Token in the ASN Source file.
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

/// A parsed token before AST is created.
///
/// Going through an ASN.1 module source results in a vector of parsed tokens of appropriate types.
/// Each parsed token contains the 'type', where it is found in the source ('span') and the actual
/// token string.
///
/// The tokens are then used by the Parser to 'resolve' type and value definitions that generates
/// the AST.
#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub span: Span,
    pub text: String,
}

impl Token {
    create_is_tokentype_fns! {
        (is_curly_begin, TokenType::CurlyBegin),
        (is_curly_end, TokenType::CurlyEnd),
        (is_round_begin, TokenType::RoundBegin),
        (is_round_end, TokenType::RoundEnd),
        (is_exception_marker, TokenType::ExceptionMarker),
        (is_square_begin, TokenType::SquareBegin),
        (is_square_end, TokenType::SquareEnd),
        (is_seq_extension_begin, TokenType::SeqExtensionBegin),
        (is_seq_extension_end, TokenType::SeqExtensionEnd),
        (is_extension, TokenType::Extension),
        (is_range_separator, TokenType::RangeSeparator),
        (is_assignment, TokenType::Assignment),
        (is_colon, TokenType::Colon),
        (is_semicolon, TokenType::SemiColon),
        (is_identifier, TokenType::Identifier),
        (is_keyword, TokenType::Keyword),
        (is_comment, TokenType::Comment),
        (is_and_identifier, TokenType::AndIdentifier),
        (is_numeric, TokenType::NumberInt),
        (is_bitstring, TokenType::BitString),
        (is_hexstring, TokenType::HexString),
        (is_tstring, TokenType::TString),
        (is_dot, TokenType::Dot),
        (is_comman, TokenType::Comma),
        (is_set_union, TokenType::SetUnion),
        (is_set_intersection, TokenType::SetIntersection),
        (is_at_component_list, TokenType::AtComponentIdList),
    }
}

#[derive(Debug)]
pub struct ObjectIdentifier;

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
