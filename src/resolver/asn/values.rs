//! Resolved 'values' implementation

use crate::error::Error;

use crate::resolver::{
    asn::structs::{
        defs::Asn1ResolvedDefinition,
        types::{base::ResolvedBaseType, Asn1ResolvedType},
        values::{
            Asn1ResolvedEnumValue, Asn1ResolvedIntegerValue, Asn1ResolvedValue, BaseEnum,
            BaseInteger, ResolvedBaseValue, ResolvedRefValue,
        },
    },
    Resolver,
};

pub(crate) fn resolve_value(
    value: &str,
    typeref: Asn1ResolvedType,
    resolver: &Resolver,
) -> Result<Asn1ResolvedValue, Error> {
    match typeref {
        Asn1ResolvedType::Base(ref b) => match b {
            ResolvedBaseType::Integer(ref _i) => {
                let value = value.parse::<BaseInteger>().unwrap();
                Ok(Asn1ResolvedValue::Base(ResolvedBaseValue::Integer(
                    Asn1ResolvedIntegerValue { typeref, value },
                )))
            }
            ResolvedBaseType::Enum(ref _e) => {
                let value = value.parse::<BaseEnum>().unwrap();
                Ok(Asn1ResolvedValue::Base(ResolvedBaseValue::Enum(
                    Asn1ResolvedEnumValue { typeref, value },
                )))
            }
            _ => Err(resolve_error!(
                "resolve_value: Not Supported Yet! {:#?}",
                typeref
            )),
        },
        Asn1ResolvedType::Reference(ref r) => {
            let typedef = resolver.resolved_defs.get(r);
            if typedef.is_none() {
                Err(resolve_error!(
                    "Definition for Reference '{}' not found or not Resolved yet!",
                    r
                ))
            } else {
                let def = typedef.unwrap();
                match def {
                    Asn1ResolvedDefinition::Type(ref t) => {
                        let v = resolve_value(value, t.clone(), resolver)?;
                        Ok(Asn1ResolvedValue::Reference(ResolvedRefValue {
                            value: Box::new(v),
                            reference: r.clone(),
                        }))
                    }
                    _ => Err(resolve_error!(
                        "Resolved Definition '{:#?}' is not a Type definition!",
                        typedef
                    )),
                }
            }
        }
        _ => Err(resolve_error!("resolve_value: Not Implemented!")),
    }
}
