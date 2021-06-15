//! Errors

#[derive(Debug)]
pub enum Error {
    TokenizeError,
    ParseError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

impl std::error::Error for Error {}
