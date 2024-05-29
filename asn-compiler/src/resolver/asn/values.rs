//! Resolved 'values' implementation

use crate::error::Error;

use crate::resolver::{
    asn::structs::{
        defs::Asn1ResolvedDefinition,
        types::{base::ResolvedBaseType, Asn1ResolvedType},
        values::{
            Asn1ResolvedEnumValue, Asn1ResolvedIntegerValue, Asn1ResolvedOidValue,
            Asn1ResolvedValue, BaseInteger, ResolvedBaseValue,
        },
    },
    Resolver,
};

pub(crate) fn resolve_value(
    value: &str,
    typeref: &Asn1ResolvedType,
    resolver: &mut Resolver,
) -> Result<Asn1ResolvedValue, Error> {
    let referenced_value = resolver.resolved_defs.get(value);
    match referenced_value {
        None => {
            match typeref {
                Asn1ResolvedType::Base(ref b) => match b {
                    ResolvedBaseType::Integer(ref _i) => {
                        let value = match value.parse::<BaseInteger>() {
                            Ok(v) => v,
                            Err(e) => {
                                return Err(resolve_error!(
                                    "resolve_value: Failed to parse Integer value: {} from {:?}",
                                    e,
                                    value
                                ))
                            }
                        };
                        Ok(Asn1ResolvedValue::Base(ResolvedBaseValue::Integer(
                            Asn1ResolvedIntegerValue {
                                typeref: typeref.clone(),
                                value,
                            },
                        )))
                    }
                    ResolvedBaseType::Enum(ref _e) => {
                        // FIXME: When Enum is resolved we'd get it right
                        // let value = value.parse::<BaseEnum>().unwrap();
                        let value = 0;
                        Ok(Asn1ResolvedValue::Base(ResolvedBaseValue::Enum(
                            Asn1ResolvedEnumValue {
                                typeref: typeref.clone(),
                                value,
                            },
                        )))
                    }
                    ResolvedBaseType::ObjectIdentifier(ref _o) => Ok(Asn1ResolvedValue::Base(
                        ResolvedBaseValue::ObjectIdentifier(Asn1ResolvedOidValue {
                            typeref: typeref.clone(),
                            value: resolve_object_identifier_value(value, resolver)?,
                        }),
                    )),
                    _ => Err(resolve_error!(
                        "resolve_value: Not Supported Yet! value: {:#?}: {:#?}",
                        value,
                        typeref
                    )),
                },
                Asn1ResolvedType::Reference(ref r) => {
                    let typedef = resolver.resolved_defs.get(r);
                    match typedef {
                        None => Err(resolve_error!(
                            "Definition for Reference '{}' not found or not Resolved yet!",
                            r
                        )),
                        Some(def) => match def {
                            Asn1ResolvedDefinition::Type(ref t) => {
                                let v = resolve_value(value, &t.clone(), resolver)?;
                                Ok(Asn1ResolvedValue::ReferencedType {
                                    value: Box::new(v),
                                    typeref: r.clone(),
                                })
                            }
                            _ => Err(resolve_error!(
                                "Resolved Definition '{:#?}' is not a Type definition!",
                                typedef
                            )),
                        },
                    }
                }
                _ => Err(resolve_error!("resolve_value: Not Implemented!")),
            }
        }
        Some(ref_value) => match ref_value {
            Asn1ResolvedDefinition::Value(ref _v) => {
                Ok(Asn1ResolvedValue::Reference(value.to_string()))
            }
            _ => Err(resolve_error!("{} Not a Referenved Value!", value)),
        },
    }
}

// Resolve the Object Identifier values For now we are simply supporting Well Known Names if
// present as named component in the 'value' string.
//
//
fn resolve_object_identifier_value(value: &str, _resolver: &Resolver) -> Result<Vec<u32>, Error> {
    use crate::parser::asn::{parse_object_identifier, WELL_KNOWN_OID_NAMES};
    use crate::tokenizer::tokenize;

    let reader = std::io::BufReader::new(std::io::Cursor::new(value));
    let tokens = tokenize(reader)?;
    let (parsed_oid, _) = parse_object_identifier(&tokens)?;

    let mut values = vec![];
    for component in parsed_oid.components {
        if component.name.is_some() {
            let name = component.name.unwrap();
            let num = WELL_KNOWN_OID_NAMES.get(name.as_str());
            if num.is_none() {
                return Err(resolve_error!(
                    "Name '{}' not found. Named component other than Well known named components not supported yet!", name));
            }
            values.push(*num.unwrap());
        } else {
            values.push(component.number);
        }
    }

    Ok(values)
}
