//! Errors

use crate::tokenizer::Token;

/// Error Type for the crate.
///
/// Defines separate variants for the Tokenization, Parsing and Symbol Resolution Errors.
#[derive(Debug)]
pub enum Error {
    /// Error when tokenizing the ASN.1 Input (Cause, Line, Column)
    TokenizeError(usize, usize, usize),

    /// Unexpected End of Tokens while parsing tokens.
    UnexpectedEndOfTokens,

    /// Unexpected Token while parsing tokens.
    UnexpectedToken(String, Token),

    /// Invalid token while parsing.
    InvalidToken(Token),

    /// Unknown Object Identifier Name (For Well known names).
    UnknownOIDName(Token),

    /// A Generic parsing error.
    ParseError(String),

    /// Error while resolving the parsed definitions.
    ResolveError(String),

    /// Error related to resolving constraints for a type.
    ConstraintError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TokenizeError(ref cause, ref l, ref c) => {
                write!(
                    f,
                    "Tokenize Error ({}) at Line: {}, Column: {}",
                    cause, l, c
                )
            }
            Error::UnexpectedEndOfTokens => {
                write!(f, "Unexpected end of tokens!")
            }
            Error::UnexpectedToken(ref un, ref tok) => {
                write!(
                    f,
                    "Expected '{}'. Found '{}' at {}.",
                    un,
                    tok.text,
                    tok.span().start()
                )
            }
            Error::InvalidToken(ref tok) => {
                write!(
                    f,
                    "Token Value '{}' is invalid at {}.",
                    tok.text,
                    tok.span().start()
                )
            }
            Error::UnknownOIDName(ref tok) => {
                write!(f,
                    "Named only Identifier '{}' in Object Identifier is not one of the well-known one at {}",
                    tok.text,
                    tok.span().start()
                )
            }
            Error::ParseError(ref errstr) => {
                write!(f, "Parsing Error: {}", errstr)
            }
            Error::ResolveError(ref errstr) => {
                write!(f, "Compilation Error: Resolve: {}", errstr)
            }
            Error::ConstraintError(ref errstr) => {
                write!(f, "Compilation Error: Constraint: {}", errstr)
            }
        }
    }
}

impl std::error::Error for Error {}

#[doc(hidden)]
impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("{}", e))
    }
}

// Macros: Use the Macros for returning Errors instead of creating the types inside any of the
// routines. This allows us to later log inside the macros if needed.
macro_rules! unexpected_token {
    ($lit: literal, $tok: expr) => {
        crate::error::Error::UnexpectedToken($lit.to_string(), $tok.clone())
    };
}

macro_rules! parse_error {
    ($($arg: tt)*) => {
        crate::error::Error::ParseError(format!($($arg)*))
    };
}

macro_rules! unexpected_end {
    () => {
        crate::error::Error::UnexpectedEndOfTokens
    };
}

macro_rules! invalid_token {
    ($tok: expr) => {
        crate::error::Error::InvalidToken($tok.clone())
    };
}

macro_rules! unknown_oid_name {
    ($tok: expr) => {
        crate::error::Error::UnknownOIDName($tok.clone())
    };
}

macro_rules! resolve_error {
    ($($arg: tt)*) => {
        crate::error::Error::ResolveError(format!($($arg)*))
    };
}

macro_rules! constraint_error {
    ($($arg: tt)*) => {
        crate::error::Error::ConstraintError(format!($($arg)*))
    };
}
