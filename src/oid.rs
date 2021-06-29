//! Structures and functions related to handling ASN.1 Object Identifier

use crate::error::Error;
use crate::tokenizer::Token;

#[derive(Debug)]
struct OIDComponent {
    name: Option<String>,
    number: u32,
}

#[derive(Debug, Default)]
pub struct ObjectIdentifier {
    components: Vec<OIDComponent>,
}

pub(crate) fn parse_object_identifier(_tokens: &[Token]) -> Result<ObjectIdentifier, Error> {
    Ok(ObjectIdentifier::default())
}
