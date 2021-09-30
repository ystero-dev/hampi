//! Token Types and related macros

use crate::tokenizer::Token;

/// Token Types
///
/// Each parsed token should be of one of the following types.
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenType {
    CurlyBegin,           // "{"
    CurlyEnd,             // "}"
    RoundBegin,           // "("
    RoundEnd,             // ")"
    ExceptionMarker,      // "!"
    SquareBegin,          // "["
    SquareEnd,            // "]"
    AdditionGroupsBegin,  // "[["
    AdditionGroupsEnd,    // "]]"
    Extension,            // "..."
    RangeSeparator,       // ".."
    Assignment,           // "::="
    Colon,                // ':'
    SemiColon,            // ';'
    Identifier,           // Identifiers and all references.
    Keyword,              // eg. "INTEGER", "ENUMERATED", "RELATIVE-OID", "TYPE-IDENTIFIER"
    Comment,              // "-- and everything after up to newline or EOF
    AndIdentifier,        // "&Attribute-Type", "&id" etc.
    NumberInt,            // eg. 123456
    BitString,            // '010...'B
    HexString,            // 'FEEDBAC...'h
    TString,              // " A string "
    Dot,                  // A single '.' usually in ATTRIBUTE.&id
    Comma,                // A single ','
    SetUnionToken,        // A single '|'
    SetIntersectionToken, // A single '^'
    AtComponentIdList,    // @Component.Id.List form
    LessThan,             // A single '<' (Used in Ranges)
}

pub(crate) type TokenChecker = fn(&Token) -> bool;

macro_rules! create_is_tokentype_fns {
    (($fn:ident, $toktype: path)) => {

        #[allow(dead_code)]
        pub(crate) fn $fn(&self) -> bool {
            self.r#type == $toktype
        }
    };

    ($($tt:tt,)*) => {
        $(
        create_is_tokentype_fns!($tt);
        )+
    };
}
