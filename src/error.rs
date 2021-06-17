//! Errors

#[derive(Debug)]
pub enum Error {
    TokenizeError(usize, usize, usize),
    ParseError,
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
            _ => {
                write!(f, "ParseError")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("{}", e))
    }
}
