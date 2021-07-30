//! Parsing of Information Object Class, Objects, Object Sets etc.
use std::collections::HashMap;

use crate::error::Error;
use crate::tokenizer::{tokenize, Token};

use crate::parser::{
    asn::values::parse_value,
    utils::{
        expect_keyword, expect_keywords, expect_one_of_keywords, expect_one_of_tokens,
        expect_token, parse_set_ish_value,
    },
};

use crate::parser::asn::structs::types::ioc::*;

use super::parse_type;

pub(crate) fn parse_class<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1ObjectClass, usize), Error> {
    let mut consumed = 0;

    if !expect_keyword(&tokens[consumed..], "CLASS")? {
        return Err(unexpected_token!("'CLASS'", tokens[consumed]));
    }
    consumed += 1;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{'", tokens[consumed]));
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
        } else {
            if expect_token(&tokens[consumed..], Token::is_curly_end)? {
                consumed += 1;
                break;
            } else {
                return Err(unexpected_token!("',' or '}'", tokens[consumed]));
            }
        }
    }

    let with_syntax_consumed = parse_with_syntax_for_fields(&tokens[consumed..], &mut fields)?;
    consumed += with_syntax_consumed;

    Ok((Asn1ObjectClass { fields }, consumed))
}

fn parse_field_spec<'parser>(
    tokens: &'parser [Token],
) -> Result<(ObjectClassFieldSpec, usize), Error> {
    if expect_token(tokens, Token::is_value_field_reference)? {
        parse_fixed_type_value_field_spec(tokens)
    } else if expect_token(tokens, Token::is_type_field_reference)? {
        parse_type_field_spec(tokens)
    } else {
        Err(parse_error!("Unsupported Field Spec in CLASS Definition"))
    }
}

fn parse_fixed_type_value_field_spec<'parser>(
    tokens: &'parser [Token],
) -> Result<(ObjectClassFieldSpec, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_value_field_reference)? {
        return Err(unexpected_token!("'VALUE FIELD REF'", tokens[consumed]));
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
            return Err(parse_error!(
                "Both 'UNIQUE' and 'DEFAULT' cannot be specified together!"
            ));
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
            resolved: false,
        },
        consumed,
    ))
}

fn parse_type_field_spec<'parser>(
    tokens: &'parser [Token],
) -> Result<(ObjectClassFieldSpec, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_type_field_reference)? {
        return Err(unexpected_token!("'TYPE FIELD REF'", tokens[consumed]));
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
            resolved: false,
        },
        consumed,
    ))
}

fn parse_with_syntax_for_fields<'parser>(
    tokens: &'parser [Token],
    fields: &'parser mut HashMap<String, ObjectClassFieldSpec>,
) -> Result<usize, Error> {
    let mut consumed = 0;
    if !expect_keywords(&tokens[consumed..], &["WITH", "SYNTAX"])? {
        return Ok(consumed);
    }
    consumed += 2;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{'", tokens[consumed]));
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
                return Err(parse_error!("Found a WITH SYNTAX RESERVED Word!"));
            }
            let words = words
                .iter()
                .map(|w| w.text.as_str())
                .collect::<Vec<&str>>()
                .join(" ");

            if !expect_token(&tokens[consumed..], Token::is_and_identifier)? {
                return Err(unexpected_token!("'CLASS field'", tokens[consumed]));
            }

            let field = fields.get_mut(&tokens[consumed].text);
            if field.is_none() {
                return Err(parse_error!(
                    "Field {} Not found in Class but found in WITH SYNTAX",
                    tokens[consumed].text
                ));
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
                    if in_optional_group && !*optional {
                        if is_default_none {
                            return Err(parse_error!(
                                "Optional Group for a field that is not Optional and No default : '{:#?}'",
                                field
                            ));
                        }
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
                return Err(unexpected_token!("',' or '}' or 'WORD'", tokens[consumed]));
            }
            in_optional_group = false;
            consumed += 1;
        }

        if expect_token(&tokens[consumed..], Token::is_curly_end)? {
            if in_optional_group {
                return Err(parse_error!("Unmatched ']' for Optional Group",));
            }
            consumed += 1;
            break;
        }
    }

    Ok(consumed)
}

pub(crate) fn parse_object_set<'parser>(
    tokens: &'parser [Token],
) -> Result<(ObjectSet, usize), Error> {
    let mut consumed = 0;

    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{'", tokens[consumed]));
    }
    consumed += 1;

    let mut root_elements = vec![];
    let mut additional_elements = vec![];
    let mut extension_token_count = 0;
    loop {
        if expect_token(&tokens[consumed..], Token::is_extension)? {
            extension_token_count += 1;
            if extension_token_count > 1 {
                return Err(parse_error!("More than one extension markers found!"));
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

        if element.is_some() {
            let element = element.unwrap();
            if extension_token_count == 0 {
                root_elements.push(element);
            } else {
                additional_elements.push(element);
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
) -> Result<(), Error> {
    let mut objectset = &mut set.objects;

    let mut root_elements = vec![];
    loop {
        let element = objectset.root_elements.pop();
        if element.is_none() {
            break;
        }
        let element = element.unwrap();
        if let ObjectSetElement::Object(ref o) = element {
            if let Asn1ObjectValue::Input(s) = o {
                let parsed = parse_object_from_class(&s, class)?;
                let element = ObjectSetElement::Object(parsed);
                root_elements.push(element);
            } else {
                root_elements.push(element);
            }
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
        if let ObjectSetElement::Object(ref o) = element {
            if let Asn1ObjectValue::Input(s) = o {
                let parsed = parse_object_from_class(&s, class)?;
                let element = ObjectSetElement::Object(parsed);
                additional_elements.push(element);
            } else {
                additional_elements.push(element);
            }
        } else {
            additional_elements.push(element);
        }
    }
    objectset.additional_elements = additional_elements;
    Ok(())
}

pub(crate) fn parse_object_from_class(
    value: &String,
    class: &Asn1ObjectClass,
) -> Result<Asn1ObjectValue, Error> {
    let reader = std::io::BufReader::new(std::io::Cursor::new(value));
    let tokens = tokenize(reader)?;
    let mut consumed = 0;
    let object_tokens = &tokens[1..tokens.len() - 1];
    let word_tokens = &mut object_tokens.split(|t| !t.is_with_syntax_word());
    let mut fields = HashMap::new();
    loop {
        let words = word_tokens.next();
        if words.is_none() {
            break;
        } else {
            let words = words.unwrap();
            consumed += words.len();
            let words = words
                .iter()
                .map(|t| t.text.clone())
                .collect::<Vec<String>>()
                .join(" ");
            let field_spec = class_fieldspec_from_words(class, &words);
            if field_spec.is_some() {
                let field_spec = field_spec.unwrap();
                let (field_spec_value, field_spec_value_consumed) =
                    value_from_field_spec(field_spec, &object_tokens[consumed..])?;
                fields.insert(field_spec.id(), field_spec_value);
                consumed += field_spec_value_consumed;
            }
        }
    }
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
    words: &'c String,
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
) -> Result<(Asn1ObjectFieldSpec, usize), Error> {
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
