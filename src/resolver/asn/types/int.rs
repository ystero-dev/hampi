use crate::error::Error;

use crate::parser::asn::structs::types::{Asn1Type, Asn1TypeKind, Asn1TypeReference};

use crate::resolver::{
    asn::{
        structs::{defs::Asn1ResolvedDefinition, types::Asn1ResolvedType},
        types::{base::resolve_base_type, constructed::resolve_constructed_type},
    },
    Resolver,
};

pub(crate) fn resolve_type(ty: &Asn1Type, resolver: &Resolver) -> Result<Asn1ResolvedType, Error> {
    match ty.kind {
        Asn1TypeKind::Builtin(ref b) => Ok(Asn1ResolvedType::Base(resolve_base_type(b, resolver)?)),
        Asn1TypeKind::Constructed(ref c) => Ok(Asn1ResolvedType::Constructed(
            resolve_constructed_type(c, resolver)?,
        )),
        Asn1TypeKind::Reference(ref r) => resolve_reference_type(r, resolver),
    }
}

fn resolve_reference_type(
    reference: &Asn1TypeReference,
    resolver: &Resolver,
) -> Result<Asn1ResolvedType, Error> {
    match reference {
        Asn1TypeReference::Reference(ref r) => {
            let resolved = resolver.resolved_defs.get(r);
            if resolved.is_some() {
                let resolved = resolved.unwrap();
                match resolved {
                    Asn1ResolvedDefinition::Type(ref _t) => {
                        Ok(Asn1ResolvedType::Reference(r.to_string()))
                    }
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
        Asn1TypeReference::Parameterized { typeref, params } => {
            let def = resolver.parameterized_defs.get(typeref);
            if def.is_some() {
                let def = def.unwrap().clone();
                let params_resolved_type = def.apply_params(params)?;
                eprintln!("params_resolved_type: {:#?}", params_resolved_type);
                resolve_type(&params_resolved_type, resolver)
                /*
                match def.kind {
                    // FIXME : we have started implementing it
                    Asn1AssignmentKind::Type(ref mut t) => {
                        if let Asn1TypeKind::Reference(ref mut r) = t.typeref.kind {
                            if let Asn1TypeReference::Parameterized { params, .. } = r {
                                *params = outer_params;
                            }
                        }
                        let resolved = resolve_type(&t.typeref, resolver);
                        resolved
                    }
                    _ => Err(resolve_error!(
                        "parameterized_type of {:#?} kind not supported.",
                        def.kind
                    )),
                }
                */
            } else {
                Err(resolve_error!(
                    "Parameterized Type for '{:#?}' Not found!",
                    reference
                ))
            }
        }
        _ => Err(resolve_error!(
            "resolve_reference_type: Not Supported: {:#?}",
            reference
        )),
    }
}
