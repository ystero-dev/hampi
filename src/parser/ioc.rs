//! Parsing of Information Object Class, Objects, Object Sets etc.

use crate::error::Error;
use crate::tokenizer::Token;

use super::utils::{expect_keywords, parse_set_ish_value};

pub(crate) fn parse_class<'parser>(tokens: &'parser [Token]) -> Result<(String, usize), Error> {
    let mut consumed = 0;

    let mut classref = String::new();
    let (def, def_consumed) = parse_set_ish_value(tokens)?;
    consumed += def_consumed;
    classref += &def;

    // FIXME: it's possible that the class definition is the last token and does not have
    // with_syntax, but fine for now
    if expect_keywords(&tokens[consumed..], &["WITH", "SYNTAX"])? {
        classref += "WITH ";
        classref += "SYNTAX ";
        consumed += 2;

        let (with_syntax, with_syntax_consumed) = parse_set_ish_value(&tokens[consumed..])?;
        classref += &with_syntax;
        consumed += with_syntax_consumed;
    }
    Ok((classref, consumed))
}
