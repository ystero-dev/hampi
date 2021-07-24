use std::collections::HashMap;

use crate::error::Error;

use crate::structs::parser::types::{Asn1Type, Asn1TypeKind, Asn1TypeReference};
use crate::structs::resolver::defs::Asn1ResolvedDefinition;
use crate::structs::resolver::types::Asn1ResolvedType;

pub(crate) mod base;
pub(crate) mod constructed;
pub(crate) mod ioc;

use base::resolve_base_type;
use constructed::resolve_constructed_type;

pub(crate) fn resolve_type(
    ty: &Asn1Type,
    table: &HashMap<String, Asn1ResolvedDefinition>,
) -> Result<Asn1ResolvedType, Error> {
    match ty.kind {
        Asn1TypeKind::Builtin(ref b) => Ok(Asn1ResolvedType::Base(resolve_base_type(b, table)?)),
        Asn1TypeKind::Constructed(ref c) => Ok(Asn1ResolvedType::Constructed(
            resolve_constructed_type(c, table)?,
        )),
        Asn1TypeKind::Reference(ref r) => resolve_reference_type(r, table),
    }
}

fn resolve_reference_type(
    reference: &Asn1TypeReference,
    table: &HashMap<String, Asn1ResolvedDefinition>,
) -> Result<Asn1ResolvedType, Error> {
    match reference {
        Asn1TypeReference::Reference(ref r) => {
            let resolved = table.get(r);
            if resolved.is_some() {
                let resolved = resolved.unwrap();
                match resolved {
                    Asn1ResolvedDefinition::Type(ref t) => Ok(t.clone()),
                    _ => Err(resolve_error!(
                        "Expected a Resolved Type, found {:#?}",
                        resolved
                    )),
                }
            } else {
                Err(resolve_error!(
                    "Referenced Type for '{}' Not resolved yet!",
                    r
                ))
            }
        }
        _ => Err(resolve_error!(
            "resolve_reference_type: Not Supported: {:#?}",
            reference
        )),
    }
}
