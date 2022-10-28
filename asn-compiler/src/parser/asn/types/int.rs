use crate::error::Error;
use crate::tokenizer::Token;

use crate::parser::asn::structs::types::{
    ActualParam, Asn1BuiltinType, Asn1ConstructedType, Asn1Type, Asn1TypeKind, Asn1TypeReference,
};

use crate::parser::utils::{expect_keywords, expect_one_of_tokens, expect_token, expect_tokens};

use super::{
    base::{parse_bitstring_type, parse_enumerated_type, parse_integer_type},
    constraints::parse_constraints,
    constructed::{parse_choice_type, parse_seq_or_seq_of_type},
};

// Parses the `Type` Expansion in the ASN.1 Grammar.
pub(crate) fn parse_type(tokens: &[Token]) -> Result<(Asn1Type, usize), Error> {
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
        "BIT" => {
            let (bitstr_type, bitstr_type_consumed) = parse_bitstring_type(tokens)?;
            (
                Asn1TypeKind::Builtin(Asn1BuiltinType::BitString(bitstr_type)),
                bitstr_type_consumed,
            )
        }

        "OCTET" => (Asn1TypeKind::Builtin(Asn1BuiltinType::OctetString), 2),

        "CHARACTER" => (
            Asn1TypeKind::Builtin(Asn1BuiltinType::CharacterString {
                str_type: "CHARACTER-STRING".to_string(),
            }),
            2,
        ),

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

        "OBJECT" => {
            if !expect_keywords(&tokens[consumed..], &["OBJECT", "IDENTIFIER"])? {
                return Err(unexpected_token!("'IDENTIFIER'", tokens[consumed + 1]));
            }

            (Asn1TypeKind::Builtin(Asn1BuiltinType::ObjectIdentifier), 2)
        }

        "BOOLEAN" => (Asn1TypeKind::Builtin(Asn1BuiltinType::Boolean), 1),

        "NULL" => (Asn1TypeKind::Builtin(Asn1BuiltinType::Null), 1),

        "VisibleString" | "UTF8String" | "IA5String" | "PrintableString" | "UTCTime"
        | "GeneralizedTime" => (
            Asn1TypeKind::Builtin(Asn1BuiltinType::CharacterString {
                str_type: typestr.to_string(),
            }),
            1,
        ),

        "CHOICE" => {
            let (choice_type, choice_type_consumed) = parse_choice_type(tokens)?;
            (
                Asn1TypeKind::Constructed(Asn1ConstructedType::Choice(choice_type)),
                choice_type_consumed,
            )
        }

        "SEQUENCE" => parse_seq_or_seq_of_type(tokens)?,

        "RELATIVE-OID" => (Asn1TypeKind::Builtin(Asn1BuiltinType::RelativeOid), 1),

        _ => parse_referenced_type(tokens)?,
    };
    consumed += kind_consumed;

    let (constraints, constraints_str_consumed) = match parse_constraints(&tokens[consumed..]) {
        Ok((s, c)) => (Some(s), c),
        Err(_) => (None, 0),
    };
    consumed += constraints_str_consumed;

    Ok((Asn1Type { kind, constraints }, consumed))
}

fn parse_referenced_type(tokens: &[Token]) -> Result<(Asn1TypeKind, usize), Error> {
    let mut consumed = 0;

    if let Ok(success) = expect_tokens(
        &tokens[consumed..],
        &[
            &[Token::is_object_class_reference],
            &[Token::is_dot],
            &[
                Token::is_type_field_reference,
                Token::is_value_field_reference,
            ],
        ],
    ) {
        if success {
            let classref = tokens[consumed].text.clone();
            let fieldref = tokens[consumed + 2].text.clone();
            return Ok((
                Asn1TypeKind::Reference(Asn1TypeReference::ClassField { classref, fieldref }),
                3,
            ));
        }
    }

    let (reference, reference_consumed) = match expect_tokens(
        &tokens[consumed..],
        &[
            &[Token::is_module_reference],
            &[Token::is_dot],
            &[Token::is_type_reference],
        ],
    ) {
        Ok(success) => {
            if success {
                (Token::concat(&tokens[consumed..consumed + 3], ""), 3)
            } else {
                (tokens[consumed].text.clone(), 1)
            }
        }
        Err(_) => (tokens[consumed].text.clone(), 1),
    };
    consumed += reference_consumed;

    let (actual_params, actual_params_consumed) =
        match expect_token(&tokens[consumed..], Token::is_curly_begin) {
            Ok(x) => {
                if x {
                    let result = parse_actual_params(&tokens[consumed..])?;
                    if result.0.is_empty() {
                        (None, result.1)
                    } else {
                        (Some(result.0), result.1)
                    }
                } else {
                    (None, 0)
                }
            }
            Err(_) => (None, 0),
        };
    consumed += actual_params_consumed;

    match actual_params {
        Some(params) => Ok((
            Asn1TypeKind::Reference(Asn1TypeReference::Parameterized {
                typeref: reference,
                params,
            }),
            consumed,
        )),
        None => Ok((
            Asn1TypeKind::Reference(Asn1TypeReference::Reference(reference)),
            consumed,
        )),
    }
}

fn parse_actual_params(tokens: &[Token]) -> Result<(Vec<ActualParam>, usize), Error> {
    let mut consumed = 0;
    if !expect_token(&tokens[consumed..], Token::is_curly_begin)? {
        return Err(unexpected_token!("'{'", tokens[consumed]));
    }
    consumed += 1;

    let mut params = vec![];
    loop {
        if expect_token(&tokens[consumed..], Token::is_curly_begin)? {
            consumed += 1;
            let param = if expect_one_of_tokens(
                &tokens[consumed..],
                &[Token::is_numeric, Token::is_identifier],
            )? {
                let param = tokens[consumed].text.clone();
                consumed += 1;
                param
            } else {
                return Err(unexpected_token!("'IDENTIFIER'", tokens[consumed]));
            };

            if !expect_token(&tokens[consumed..], Token::is_curly_end)? {
                return Err(unexpected_token!("'}'", tokens[consumed]));
            };
            consumed += 1;
            params.push(ActualParam::Set(param));
        }

        if expect_one_of_tokens(
            &tokens[consumed..],
            &[Token::is_numeric, Token::is_identifier],
        )? {
            let param = tokens[consumed].text.clone();
            consumed += 1;
            params.push(ActualParam::Single(param));
        }
        if expect_token(&tokens[consumed..], Token::is_comma)? {
            consumed += 1;
        }
        if expect_token(&tokens[consumed..], Token::is_curly_end)? {
            consumed += 1;
            break;
        }
    }
    Ok((params, consumed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn parse_type_test() {
        struct ParseTypeTestCase<'tc> {
            input: &'tc str,
            success: bool,
            consumed: usize,
        }

        let test_cases = vec![
            ParseTypeTestCase {
                input: "OBJECT IDENTIFIER",
                success: true,
                consumed: 2,
            },
            ParseTypeTestCase {
                input: "SEQUENCE SIZE (1.. 1024) OF ReportData",
                success: true,
                consumed: 9,
            },
            ParseTypeTestCase {
                input: "CHOICE { absoluteTime  UTCTime, relativeTime  INTEGER (0..31536000)}",
                success: true,
                consumed: 13,
            },
        ];

        for tc in test_cases {
            let reader = std::io::BufReader::new(std::io::Cursor::new(tc.input));
            let tokens = tokenize(reader);
            assert!(tokens.is_ok());
            let tokens = tokens.unwrap();

            let ty = parse_type(&tokens);
            assert_eq!(
                ty.is_ok(),
                tc.success,
                "{}:{}",
                tc.input,
                if tc.success {
                    format!("{:#?}", ty.err())
                } else {
                    format!("{:#?}", ty.ok())
                }
            );

            if tc.success {
                let (_, ty_consumed) = ty.unwrap();
                assert_eq!(ty_consumed, tc.consumed, "{}", tc.input);
            }
        }
    }
}
