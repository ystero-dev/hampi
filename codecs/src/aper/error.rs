//! APER Codec Errors
//!
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new<T: AsRef<str> + Display>(msg: T) -> Self {
        Error {
            msg: msg.to_string(),
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {}
