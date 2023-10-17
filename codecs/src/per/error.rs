//! APER Codec Errors
//!
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    /// `ErrorCause` can be used to take actions based on the cause.
    pub cause: ErrorCause,
    msg: String,
    context: Vec<String>,
}

#[derive(Debug)]
pub enum ErrorCause {
    /// Remaining Buffer is too short to decode
    BufferTooShort,

    /// Alignment Error during decode
    InvalidAlignment,

    /// Encoding of value is not currently supported
    EncodeNotSupported,

    /// Decoding of value is not currently supported
    DecodeNotSupported,

    /// Generic Error during Encode or Decode
    Generic,
}

impl Error {
    pub fn new<T: AsRef<str> + Display>(cause: ErrorCause, msg: T) -> Self {
        Error {
            cause,
            msg: msg.to_string(),
            context: Vec::new(),
        }
    }
    pub fn push_context(&mut self, context_elem: &str) {
        self.context.push(context_elem.to_string());
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.context.is_empty() {
            write!(f, "cause: {}, msg: {}", self.cause, self.msg)
        } else {
            write!(
                f,
                "cause: {}, msg: [{}]:{}",
                self.cause,
                self.context.join("."),
                self.msg
            )
        }
    }
}

impl std::fmt::Display for ErrorCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}
