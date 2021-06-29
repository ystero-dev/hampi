//! Structures and functions related to handling ASN.1 Object Identifier

use crate::error::Error;
use crate::tokenizer::{expect_token, Token};

#[derive(Debug)]
struct OIDComponent {
    name: Option<String>,
    number: u32,
}

#[derive(Debug, Default)]
pub struct ObjectIdentifier {
    components: Vec<OIDComponent>,
}

fn parse_named_oid_component<'parser>(
    tokens: &'parser [Token],
) -> Result<(OIDComponent, usize), Error> {
    if tokens.len() == 0 {
        return Err(Error::ParseError);
    }

    if tokens.len() < 4 {
        let token = &tokens[0];
        return Ok((
            OIDComponent {
                name: Some(token.text.clone()),
                number: 1,
            },
            1,
        ));
    }

    let (tok1, tok2, tok3, tok4) = (&tokens[0], &tokens[1], &tokens[2], &tokens[3]);
    if tok2.is_round_begin() && tok3.is_numeric() && tok4.is_round_end() {
        return Ok((
            OIDComponent {
                name: Some(tok1.text.clone()),
                number: tok3.text.parse::<u32>().map_err(|_| Error::ParseError)?,
            },
            4,
        ));
    } else {
        return Ok((
            OIDComponent {
                name: Some(tok1.text.clone()),
                number: 1,
            },
            1,
        ));
    }
}

fn parse_oid_component<'parser>(tokens: &'parser [Token]) -> Result<(OIDComponent, usize), Error> {
    if tokens.len() == 0 {
        return Err(Error::ParseError);
    }

    let first = &tokens[0];
    if first.is_identifier() {
        eprintln!("first: {:#?}", first);
        parse_named_oid_component(tokens)
    } else if first.is_numeric() {
        let number = first.text.parse::<u32>().map_err(|_| Error::ParseError)?;
        Ok((OIDComponent { name: None, number }, 1))
    } else {
        Err(Error::ParseError)
    }
}

pub(crate) fn parse_object_identifier<'parser>(
    tokens: &'parser [Token],
) -> Result<ObjectIdentifier, Error> {
    let t: &Token;
    let mut consumed = 0;

    t = &tokens[consumed];
    if !expect_token(t, Token::is_curly_begin) {
        eprintln!("Not curly begin");
        return Err(Error::ParseError);
    }
    consumed += 1;

    let mut components = vec![];
    loop {
        let result = parse_oid_component(&tokens[consumed..]);
        match result {
            Ok((comp, c)) => {
                components.push(comp);
                consumed += c;
            }
            Err(e) => {
                let tok = &tokens[consumed];
                if tok.is_curly_end() {
                    break;
                } else {
                    eprintln!("{:#?}", tok);
                    return Err(e);
                }
            }
        }
    }
    Ok(ObjectIdentifier { components })
}
