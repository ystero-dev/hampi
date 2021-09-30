//! APER Codec Errors
//!
#[derive(Debug)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "aper_error!")
    }
}

impl std::error::Error for Error {}
