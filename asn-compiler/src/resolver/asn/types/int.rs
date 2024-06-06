use anyhow::Result;

use crate::parser::asn::structs::types::{
    Asn1BuiltinType, Asn1Type, Asn1TypeKind, Asn1TypeReference,
};

use crate::resolver::{
    asn::{
        structs::{
            defs::Asn1ResolvedDefinition,
            types::{constraints::Asn1ConstraintValueSet, Asn1ResolvedType},
        },
        types::{base::resolve_base_type, constructed::resolve_constructed_type},
    },
    Resolver,
};

impl Asn1Type {
    // Returns the Integer ValueSet for a given Type.
    //
    // If the type is a `Base` type, it should be INTEGER or it's an Error. If the `Type` is a
    // Referenced Type, it should be possible to 'resolve' the Reference to a proper
    // `Asn1ConstraintValueSet` else it's an error, we'll try to 'recursively' 'resolve' till we
    // get to a `Base` Type.
    pub(crate) fn get_integer_valueset_from_constraint(
        &self,
        resolver: &Resolver,
    ) -> Result<Asn1ConstraintValueSet> {
        let kind = &self.kind;
        match kind {
            Asn1TypeKind::Builtin(Asn1BuiltinType::Integer(..)) => {
                let constraint = &self.constraints.as_ref().unwrap()[0];
                constraint.get_integer_valueset(resolver)
            }
            Asn1TypeKind::Reference(Asn1TypeReference::Reference(ref _r)) => {
                Err(constraint_error!("Not Implemented!").into())
            }
            _ => Err(constraint_error!(
                "The Type '{:#?}' is not of a BuiltIn Or a Referenced Kind!",
                self,
            )
            .into()),
        }
    }
}

pub(crate) fn resolve_type(ty: &Asn1Type, resolver: &mut Resolver) -> Result<Asn1ResolvedType> {
    match ty.kind {
        Asn1TypeKind::Builtin(..) => Ok(Asn1ResolvedType::Base(resolve_base_type(ty, resolver)?)),
        Asn1TypeKind::Constructed(..) => resolve_constructed_type(ty, resolver),

        Asn1TypeKind::Reference(..) => resolve_reference_type(ty, resolver),
    }
}

fn resolve_reference_type(ty: &Asn1Type, resolver: &mut Resolver) -> Result<Asn1ResolvedType> {
    if let Asn1TypeKind::Reference(ref reference) = ty.kind {
        match reference {
            Asn1TypeReference::Reference(ref r) => {
                let resolved = resolver.resolved_defs.get(r);
                match resolved {
                    Some(res) => match res {
                        Asn1ResolvedDefinition::Type(..) => {
                            Ok(Asn1ResolvedType::Reference(r.to_string()))
                        }
                        _ => Err(
                            resolve_error!("Expected a Resolved Type, found {:#?}", resolved)
                                .into(),
                        ),
                    },
                    None => {
                        Err(resolve_error!("Referenced Type for '{}' Not resolved yet!", r).into())
                    }
                }
            }
            Asn1TypeReference::Parameterized { typeref, params } => {
                let def = resolver.parameterized_defs.get(typeref);
                match def {
                    Some(d) => {
                        let params_resolved_type = d.clone().apply_params(params)?;
                        resolve_type(&params_resolved_type, resolver)
                    }
                    None => Err(resolve_error!(
                        "Parameterized Type for '{:#?}' Not found!",
                        reference
                    )
                    .into()),
                }
            }
            Asn1TypeReference::ClassField { .. } => {
                Err(resolve_error!("Supported Inside Constructed Sequence Type.").into())
            }
        }
    } else {
        Err(resolve_error!("Expected Reference Type. Found '{:#?}'", ty).into())
    }
}
