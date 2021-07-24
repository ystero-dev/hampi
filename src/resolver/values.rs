//! Resolved 'values' implementation

use crate::error::Error;

use crate::structs::resolver::types::{base::ResolvedBaseType, Asn1ResolvedType};
use crate::structs::resolver::values::{
    Asn1ResolvedEnumValue, Asn1ResolvedIntegerValue, Asn1ResolvedValue, BaseEnum, BaseInteger,
};

pub(crate) fn resolve_value(
    value: &str,
    typeref: Asn1ResolvedType,
) -> Result<Asn1ResolvedValue, Error> {
    match typeref {
        Asn1ResolvedType::Base(ref b) => match b {
            ResolvedBaseType::Integer(ref _i) => {
                let value = value.parse::<BaseInteger>().unwrap();
                Ok(Asn1ResolvedValue::Integer(Asn1ResolvedIntegerValue {
                    typeref,
                    value,
                }))
            }
            ResolvedBaseType::Enum(ref _e) => {
                let value = value.parse::<BaseEnum>().unwrap();
                Ok(Asn1ResolvedValue::Enum(Asn1ResolvedEnumValue {
                    typeref,
                    value,
                }))
            }
            _ => Err(resolve_error!(
                "resolve_value: Not Supported Yet! {:#?}",
                typeref
            )),
        },
        _ => Err(resolve_error!("resolve_value: Not Implemented!")),
    }
}
