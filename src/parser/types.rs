//! Handling Parsing of ASN.1 Types

use crate::error::Error;
use crate::structs::types::{
    Asn1BuiltinType, Asn1ConstructedType, Asn1Type, Asn1TypeKind, ASN_BUILTIN_TYPE_KINDS,
};
use crate::tokenizer::Token;

use super::base::{parse_enumerated_type, parse_integer_type};
use super::constructed::parse_choice_type;

use super::constraints::parse_constraints;

use super::utils::{expect_one_of_keywords, expect_one_of_tokens, parse_set_ish_value};

// Parses the `Type` Expansion in the ASN.1 Grammar.
pub(super) fn parse_type<'parser>(tokens: &'parser [Token]) -> Result<(Asn1Type, usize), Error> {
    let mut consumed = 0;

    if !expect_one_of_tokens(
        tokens,
        &[
            Token::is_type_reference,
            Token::is_asn_builtin_type,
            Token::is_object_class_reference,
        ],
    )? {
        return Err(unexpected_token!(
            "'Type Reference' or 'Builtin Type'",
            tokens[0]
        ));
    }

    // Now: Parse The Type definition.
    let token = &tokens[0];
    let typestr = token.text.as_str();
    let (kind, kind_consumed) = match typestr {
        "BIT-STRING" => {
            let (_, id_consumed) = parse_bit_string_type(tokens)?;
            (
                ASN_BUILTIN_TYPE_KINDS.get(typestr).unwrap().clone(),
                id_consumed,
            )
        }

        "ENUMERATED" => {
            let (enum_type, enum_type_consumed) = parse_enumerated_type(tokens)?;
            (
                Asn1TypeKind::Builtin(Asn1BuiltinType::Enumerated(enum_type)),
                enum_type_consumed,
            )
        }

        "INTEGER" => {
            let (int_type, int_type_consumed) = parse_integer_type(tokens)?;
            (
                Asn1TypeKind::Builtin(Asn1BuiltinType::Integer(int_type)),
                int_type_consumed,
            )
        }

        "BOOLEAN" | "NULL" | "OBJECT-IDENTIFIER" | "UTF8String" | "IA5String"
        | "PrintableString" | "CHARACTER-STRING" => {
            (ASN_BUILTIN_TYPE_KINDS.get(typestr).unwrap().clone(), 1)
        }

        "CHOICE" => {
            let (choice_type, choice_type_consumed) = parse_choice_type(tokens)?;
            (
                Asn1TypeKind::Constructed(Asn1ConstructedType::Choice(choice_type)),
                choice_type_consumed,
            )
        }
        "SET" | "SEQUENCE" => parse_constructed_type(tokens)?,

        _ => (Asn1TypeKind::default(), 1),
    };
    consumed += kind_consumed;

    let (constraints, constraints_str_consumed) = match parse_constraints(&tokens[consumed..]) {
        Ok((s, c)) => (Some(s), c),
        Err(_) => (None, 0),
    };
    consumed += constraints_str_consumed;

    Ok((Asn1Type { kind, constraints }, consumed))
}

fn parse_bit_string_type<'parser>(_tokens: &'parser [Token]) -> Result<(String, usize), Error> {
    Err(parse_error!("Not Implemented yet!"))
}

fn parse_constructed_type<'parser>(
    tokens: &'parser [Token],
) -> Result<(Asn1TypeKind, usize), Error> {
    let mut consumed = 0;

    if !expect_one_of_keywords(tokens, &["SEQUENCE", "SET", "CHOICE"])? {
        return Err(unexpected_token!("'SEQUENCE', 'SET', 'CHOICE'", tokens[0]));
    }

    let (_, def_consumed) = parse_set_ish_value(&tokens[consumed..])?;
    consumed += def_consumed;

    Ok((
        Asn1TypeKind::Constructed(Asn1ConstructedType::Sequence),
        consumed,
    ))
}

// TODO: Add test cases
