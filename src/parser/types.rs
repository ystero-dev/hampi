//! Handling Parsing of ASN.1 Types

use crate::error::Error;
use crate::parser::expect_token_one_of;
use crate::tokenizer::Token;

// Parses the `Type` Expansion in the ASN.1 Grammar.
pub fn parse_type<'parser>(tokens: &'parser [Token]) -> Result<(String, usize), Error> {
    if expect_token_one_of(
        tokens,
        &[Token::is_type_reference, Token::is_asn_builtin_type],
    )? {
        return Ok((tokens[0].text.clone(), 1));
    }

    eprintln!("token: {:#?}", tokens[0]);
    Err(parse_error!("Unsupported Type Expansion Parsing!"))
}
