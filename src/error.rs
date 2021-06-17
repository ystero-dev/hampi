//! Errors

#[derive(Debug)]
pub enum Error {
    TokenizeError(usize, usize),
    ParseError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TokenizeError(ref l, ref c) => {
                write!(f, "Tokenize Error at Line: {}, Column: {}", l, c)
            }
            _ => {
                write!(f, "ParseError")
            }
        }
    }
}

impl std::error::Error for Error {}
