//! Parsing of Information Object Class, Objects, Object Sets etc.
use std::collections::HashMap;

use crate::tokenizer::{tokenize, Token};
use anyhow::Result;

use crate::parser::{
    asn::values::parse_value,
    utils::{
        expect_keyword, expect_keywords, expect_one_of_keywords, expect_one_of_tokens,
        expect_token, parse_set_ish_value,
    },
};

use crate::parser::asn::structs::types::ioc::*;

use super::parse_type;

pub(crate) fn parse_class(tokens: &[Token]) -> Result<(Asn1ObjectClass, usize)> {
    let mut consumed = 0;

    if !expect_keyword(&tokens[consumed..], "CLASS")? {
        return Err(unexpected_token!("'CLASS'", tokens[consumed]).into());
    }
    consumed += 1;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{'", tokens[consumed]).into());
    }
    consumed += 1;

    let mut fields = HashMap::new();
    loop {
        // Any error in parsing a field spec is an error.
        let (field_spec, field_spec_consumed) = parse_field_spec(&tokens[consumed..])?;
        consumed += field_spec_consumed;

        fields.insert(field_spec.id(), field_spec);

        if expect_token(&tokens[consumed..], Token::is_comma)? {
            consumed += 1;
        } else if expect_token(&tokens[consumed..], Token::is_curly_end)? {
            consumed += 1;
            break;
        } else {
            return Err(unexpected_token!("',' or '}'", tokens[consumed]).into());
        }
    }

    let with_syntax_consumed = parse_with_syntax_for_fields(&tokens[consumed..], &mut fields)?;
    consumed += with_syntax_consumed;

    Ok((Asn1ObjectClass { fields }, consumed))
}

fn parse_field_spec(tokens: &[Token]) -> Result<(ObjectClassFieldSpec, usize)> {
    if expect_token(tokens, Token::is_value_field_reference)? {
        parse_fixed_type_value_field_spec(tokens)
    } else if expect_token(tokens, Token::is_type_field_reference)? {
        parse_type_field_spec(tokens)
    } else {
        Err(parse_error!("Unsupported Field Spec in CLASS Definition").into())
    }
}

fn parse_fixed_type_value_field_spec(tokens: &[Token]) -> Result<(ObjectClassFieldSpec, usize)> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_value_field_reference)? {
        return Err(unexpected_token!("'VALUE FIELD REF'", tokens[consumed]).into());
    }

    let id = tokens[consumed].text.clone();
    consumed += 1;

    let (field_type, field_type_consumed) = parse_type(&tokens[consumed..])?;
    consumed += field_type_consumed;

    let unique = match expect_keyword(&tokens[consumed..], "UNIQUE") {
        Ok(u) => {
            if u {
                consumed += 1;
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };

    let mut optional = false;
    let mut default = None;
    if expect_one_of_keywords(&tokens[consumed..], &["OPTIONAL", "DEFAULT"])? {
        if expect_keyword(&tokens[consumed..], "OPTIONAL")? {
            optional = match expect_keyword(&tokens[consumed..], "OPTIONAL") {
                Ok(o) => {
                    if o {
                        consumed += 1;
                        true
                    } else {
                        false
                    }
                }
                Err(_) => false,
            };
        } else {
            default = match expect_keyword(&tokens[consumed..], "DEFAULT") {
                Ok(x) => {
                    if x {
                        consumed += 1;
                        let (value, value_consumed) = parse_value(&tokens[consumed..])?;
                        consumed += value_consumed;
                        Some(value)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            };
        }

        if default.is_some() && unique {
            return Err(
                parse_error!("Both 'UNIQUE' and 'DEFAULT' cannot be specified together!").into(),
            );
        }
    }

    let with_syntax = None;

    Ok((
        ObjectClassFieldSpec::FixedTypeValue {
            id,
            field_type,
            unique,
            optional,
            default,
            with_syntax,
            _resolved: false,
        },
        consumed,
    ))
}

fn parse_type_field_spec(tokens: &[Token]) -> Result<(ObjectClassFieldSpec, usize)> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_type_field_reference)? {
        return Err(unexpected_token!("'TYPE FIELD REF'", tokens[consumed]).into());
    }

    let id = tokens[consumed].text.clone();
    consumed += 1;

    let mut optional = false;
    let mut default = None;
    if expect_one_of_keywords(&tokens[consumed..], &["OPTIONAL", "DEFAULT"])? {
        if expect_keyword(&tokens[consumed..], "OPTIONAL")? {
            optional = match expect_keyword(&tokens[consumed..], "OPTIONAL") {
                Ok(o) => {
                    if o {
                        consumed += 1;
                        true
                    } else {
                        false
                    }
                }
                Err(_) => false,
            };
        } else {
            default = match expect_keyword(&tokens[consumed..], "DEFAULT") {
                Ok(x) => {
                    if x {
                        consumed += 1;
                        let (default, default_consumed) = parse_type(&tokens[consumed..])?;
                        consumed += default_consumed;
                        Some(default)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            };
        }
    }

    let with_syntax = None;

    Ok((
        ObjectClassFieldSpec::Type {
            id,
            optional,
            default,
            with_syntax,
            _resolved: false,
        },
        consumed,
    ))
}

fn parse_with_syntax_for_fields(
    tokens: &[Token],
    fields: &mut HashMap<String, ObjectClassFieldSpec>,
) -> Result<usize> {
    let mut consumed = 0;
    if !expect_keywords(&tokens[consumed..], &["WITH", "SYNTAX"])? {
        return Ok(consumed);
    }
    consumed += 2;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{'", tokens[consumed]).into());
    }
    consumed += 1;

    let mut in_optional_group = false;
    loop {
        if expect_token(&tokens[consumed..], Token::is_square_begin)? {
            in_optional_group = true;
            consumed += 1;
        }

        let words = &tokens[consumed..].split(Token::is_and_identifier).next();
        if words.is_some() {
            // A slice of tokens
            let words = words.unwrap();
            consumed += words.len();
            if words.iter().any(Token::is_with_syntax_reserved_word) {
                return Err(parse_error!("Found a WITH SYNTAX RESERVED Word!").into());
            }
            let words = words
                .iter()
                .map(|w| w.text.as_str())
                .collect::<Vec<&str>>()
                .join(" ");

            if !expect_token(&tokens[consumed..], Token::is_and_identifier)? {
                return Err(unexpected_token!("'CLASS field'", tokens[consumed]).into());
            }

            let field = fields.get_mut(&tokens[consumed].text);
            if field.is_none() {
                return Err(parse_error!(
                    "Field {} Not found in Class but found in WITH SYNTAX",
                    tokens[consumed].text
                )
                .into());
            }
            consumed += 1;

            let field = field.unwrap();

            let is_default_none = match field {
                ObjectClassFieldSpec::Type { default, .. } => default.is_none(),
                ObjectClassFieldSpec::FixedTypeValue { default, .. } => default.is_none(),
            };
            match field {
                ObjectClassFieldSpec::Type {
                    with_syntax,
                    optional,
                    ..
                }
                | ObjectClassFieldSpec::FixedTypeValue {
                    with_syntax,
                    optional,
                    ..
                } => {
                    if in_optional_group && !*optional && is_default_none {
                        return Err(parse_error!(
                                "Optional Group for a field that is not Optional and No default : '{:#?}'",
                                field
                            ).into());
                    }
                    *with_syntax = Some(words);
                }
            }
        }

        // Sometimes you may see a comma after the Class Field, just consume it.
        if expect_token(&tokens[consumed..], Token::is_comma)? {
            consumed += 1;
        }

        if expect_token(&tokens[consumed..], Token::is_square_end)? {
            if !in_optional_group {
                return Err(unexpected_token!("',' or '}' or 'WORD'", tokens[consumed]).into());
            }
            in_optional_group = false;
            consumed += 1;
        }

        if expect_token(&tokens[consumed..], Token::is_curly_end)? {
            if in_optional_group {
                return Err(parse_error!("Unmatched ']' for Optional Group",).into());
            }
            consumed += 1;
            break;
        }
    }

    Ok(consumed)
}

pub(crate) fn parse_object_set(tokens: &[Token]) -> Result<(ObjectSet, usize)> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{'", tokens[consumed]).into());
    }
    consumed += 1;

    let mut root_elements = vec![];
    let mut additional_elements = vec![];
    let mut extension_token_count = 0;
    loop {
        if expect_token(&tokens[consumed..], Token::is_extension)? {
            extension_token_count += 1;
            if extension_token_count > 1 {
                return Err(parse_error!("More than one extension markers found!").into());
            }
            consumed += 1;
            if expect_token(&tokens[consumed..], Token::is_comma)? {
                consumed += 1;
            }
        }

        let element = match parse_set_ish_value(&tokens[consumed..]) {
            Ok(result) => {
                let (value, value_consumed) = result;

                consumed += value_consumed;
                Some(ObjectSetElement::Object(Asn1ObjectValue::Input(value)))
            }
            Err(_) => {
                // It may be a reference to an object set, allowed
                if expect_one_of_tokens(
                    &tokens[consumed..],
                    &[Token::is_object_set_reference, Token::is_object_reference],
                )? {
                    let token = &tokens[consumed];
                    consumed += 1;
                    if token.is_object_reference() {
                        Some(ObjectSetElement::ObjectReference(token.text.clone()))
                    } else {
                        Some(ObjectSetElement::ObjectSetReference(token.text.clone()))
                    }
                } else {
                    None
                }
            } // Empty Values permitted
        };

        if let Some(ele) = element {
            if extension_token_count == 0 {
                root_elements.push(ele);
            } else {
                additional_elements.push(ele);
            }
        }

        if expect_token(&tokens[consumed..], Token::is_comma)? {
            consumed += 1;
        }

        if expect_token(&tokens[consumed..], Token::is_set_union)? {
            consumed += 1;
        }

        if expect_token(&tokens[consumed..], Token::is_curly_end)? {
            consumed += 1;
            break;
        }
    }
    Ok((
        ObjectSet {
            root_elements,
            additional_elements,
        },
        consumed,
    ))
}

pub(crate) fn parse_object_set_from_class(
    set: &mut Asn1ObjectSet,
    class: &Asn1ObjectClass,
) -> Result<()> {
    let objectset = &mut set.objects;

    let mut root_elements = vec![];
    loop {
        let element = objectset.root_elements.pop();
        if element.is_none() {
            break;
        }
        let element = element.unwrap();
        if let ObjectSetElement::Object(Asn1ObjectValue::Input(s)) = element {
            let parsed = parse_object_from_class(&s, class)?;
            let element = ObjectSetElement::Object(parsed);
            root_elements.push(element);
        } else {
            root_elements.push(element);
        }
    }
    objectset.root_elements = root_elements;

    // FIXME : Make it a function?
    let mut additional_elements = vec![];
    loop {
        let element = objectset.additional_elements.pop();
        if element.is_none() {
            break;
        }
        let element = element.unwrap();
        if let ObjectSetElement::Object(Asn1ObjectValue::Input(s)) = element {
            let parsed = parse_object_from_class(&s, class)?;
            let element = ObjectSetElement::Object(parsed);
            additional_elements.push(element);
        } else {
            additional_elements.push(element);
        }
    }
    objectset.additional_elements = additional_elements;
    Ok(())
}

// Parse an input value - which is so far a 'string' into the respective `Asn1ObjectValue`
//
// Typically parses Something like
// { Sometype IDENTIFIED BY someID} or
// { WORDS FOR TYPE Type MORE WORDS value} etc. values
//
// Values prefexing `Type` or `value` are associated with `with_syntax` for the type. In the case
// of `IDENTIFIED BY` syntax The prefix is an 'empty' string.
pub(crate) fn parse_object_from_class(
    value: &str,
    class: &Asn1ObjectClass,
) -> Result<Asn1ObjectValue> {
    let reader = std::io::BufReader::new(std::io::Cursor::new(value));
    let tokens = tokenize(reader)?;
    let mut consumed = 0;
    let object_tokens = &tokens[1..tokens.len() - 1];

    // We are splitting the tokens by words from WITH SYNTAX. This will use any word that's not
    // there in WITH SYNTAX. This is a bit complicated because we've to also worry about WITH
    // SYNTAX words that can be more than a single WORD.
    let word_tokens = &mut object_tokens.split(|t| {
        !class
            .get_with_syntax_words()
            .iter()
            .any(|f| f.split_whitespace().any(|w| t.text == w))
    });
    let mut fields = HashMap::new();
    for words in word_tokens {
        consumed += words.len();
        let words = words
            .iter()
            .map(|t| t.text.clone())
            .collect::<Vec<String>>()
            .join(" ");
        let field_spec = class_fieldspec_from_words(class, &words);
        if let Some(fspec) = field_spec {
            let (field_spec_value, field_spec_value_consumed) =
                value_from_field_spec(fspec, &object_tokens[consumed..])?;
            fields.insert(fspec.id(), field_spec_value);
            consumed += field_spec_value_consumed;

            // Required to handle &Type IDENTIFIED BY &id
            if consumed == object_tokens.len() {
                break;
            }
        }
    }

    // If we didn't find an optional value, Check if default exists, if default exists, give it to
    // the resolved value.
    for (key, spec) in class.fields.iter() {
        if !fields.contains_key(key) {
            match spec {
                ObjectClassFieldSpec::Type { default, .. } => {
                    if default.is_some() {
                        fields.insert(
                            key.clone(),
                            Asn1ObjectFieldSpec::Type {
                                ty: default.clone(),
                            },
                        );
                    }
                }
                ObjectClassFieldSpec::FixedTypeValue {
                    default,
                    field_type,
                    ..
                } => {
                    if default.is_some() {
                        fields.insert(
                            key.clone(),
                            Asn1ObjectFieldSpec::FixedTypeValue {
                                typeref: field_type.clone(),
                                value: default.clone(),
                            },
                        );
                    }
                }
            }
        }
    }
    // TODO : Handle Default ones that are notyet processed.
    Ok(Asn1ObjectValue::Asn1ObjectFromClass { fields })
}

fn class_fieldspec_from_words<'c>(
    class: &'c Asn1ObjectClass,
    words: &str,
) -> Option<&'c ObjectClassFieldSpec> {
    for field in class.fields.values() {
        match field {
            ObjectClassFieldSpec::Type { with_syntax, .. }
            | ObjectClassFieldSpec::FixedTypeValue { with_syntax, .. } => {
                if with_syntax.is_none() {
                    continue;
                }
                let with_syntax = with_syntax.as_ref().unwrap();
                if with_syntax == words {
                    return Some(field);
                }
            }
        }
    }
    None
}

fn value_from_field_spec(
    spec: &ObjectClassFieldSpec,
    tokens: &[Token],
) -> Result<(Asn1ObjectFieldSpec, usize)> {
    match spec {
        ObjectClassFieldSpec::Type { .. } => {
            let (ty, ty_consumed) = parse_type(tokens)?;
            Ok((Asn1ObjectFieldSpec::Type { ty: Some(ty) }, ty_consumed))
        }
        ObjectClassFieldSpec::FixedTypeValue { field_type, .. } => {
            let (value, value_consumed) = parse_value(tokens)?;
            Ok((
                Asn1ObjectFieldSpec::FixedTypeValue {
                    typeref: field_type.clone(),
                    value: Some(value),
                },
                value_consumed,
            ))
        }
    }
}
