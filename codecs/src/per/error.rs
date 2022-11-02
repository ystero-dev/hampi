//! APER Codec Errors
//!
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    msg: String,
    context: Vec<String>,
}

impl Error {
    pub fn new<T: AsRef<str> + Display>(msg: T) -> Self {
        Error {
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
            write!(f, "{}", self.msg)
        } else {
            write!(f, "[{}]:{}", self.context.join("."), self.msg)
        }
    }
}

impl std::error::Error for Error {}
