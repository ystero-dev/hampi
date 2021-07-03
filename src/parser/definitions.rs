//! Top level handling of definitions

use crate::error::Error;
use crate::parser::{expect_token_one_of, expect_tokens};
use crate::structs::{Asn1Definition, Asn1ValueAssignment};
use crate::tokenizer::Token;

pub(crate) fn parse_definition<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Definition, usize), Error> {
    let consumed = 0;

    if expect_token_one_of(
        &tokens[consumed..],
        &[Token::is_value_reference, Token::is_type_reference],
    )? {
        let next = &tokens[consumed];
        if next.is_value_reference() {
            parse_value_definition(&tokens[consumed..])
        } else {
            parse_typeish_definition(&tokens[consumed..])
        }
    } else {
        eprintln!("Token: {:#?})", tokens[consumed]);
        Err(parse_error!("Not Implemented!"))
    }
}

fn parse_value_definition<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Definition, usize), Error> {
    if expect_tokens(
        tokens,
        &[
            &[Token::is_value_reference],
            &[Token::is_type_reference, Token::is_asn_base_type],
            &[Token::is_assignment],
        ],
    )? {
        if expect_token_one_of(&tokens[3..], &[Token::is_identifier, Token::is_numeric])? {
            let id = tokens[0].text.clone();
            let typeref = tokens[1].text.clone();
            let text = tokens[3].text.clone();
            Ok((
                Asn1Definition::Value(Asn1ValueAssignment { id, typeref, text }),
                4,
            ))
        } else {
            Err(unexpected_token!(
                "Expected 'valuereference Typereference ::=",
                tokens[0]
            ))
        }
    } else {
        Err(unexpected_token!(
            "Expected 'valuereference Typereference ::=",
            tokens[0]
        ))
    }
}

fn parse_typeish_definition<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1Definition, usize), Error> {
    eprintln!("Token: {:#?}", tokens[0]);
    Err(parse_error!("Typeish Assignment Parsing Not Implemented"))
}
