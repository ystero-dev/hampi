//! Token Types and related macros

/// Token Types
///
/// Each parsed token should be of one of the following types.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    CurlyBegin,        // "{"
    CurlyEnd,          // "}"
    RoundBegin,        // "("
    RoundEnd,          // ")"
    ExceptionMarker,   // "!"
    SquareBegin,       // "["
    SquareEnd,         // "]"
    SeqExtensionBegin, // "[["
    SeqExtensionEnd,   // "]]"
    Extension,         // "..."
    RangeSeparator,    // ".."
    Assignment,        // "::="
    Colon,             // ':'
    SemiColon,         // ';'
    Identifier,        // Identifiers and all references.
    Keyword,           // eg. "INTEGER", "ENUMERATED", "RELATIVE-OID", "TYPE-IDENTIFIER"
    Comment,           // "-- and everything after up to newline or EOF
    AndIdentifier,     // "&Attribute-Type", "&id" etc.
    NumberInt,         // eg. 123456
    BitString,         // '010...'B
    HexString,         // 'FEEDBAC...'h
    TString,           // " A string "
    Dot,               // A single '.' usually in ATTRIBUTE.&id
    Comma,             // A single ','
    SetUnion,          // A single '|'
    SetIntersection,   // A single '^'
    AtComponentIdList, // @Component.Id.List form
}

macro_rules! create_is_tokentype_fns {
    (($fn:ident, $toktype: path)) => {
        pub fn $fn(&self) -> bool {
            self.r#type == $toktype
        }
    };

    ($($tt:tt,)*) => {
        $(
        create_is_tokentype_fns!($tt);
        )+
    };
}
