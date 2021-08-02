use crate::error::Error;

use crate::parser::asn::structs::types::{
    constructed::{Asn1TypeChoice, Asn1TypeSequence, Asn1TypeSequenceOf, Component},
    Asn1ConstructedType, Asn1Type, Asn1TypeKind, Asn1TypeReference,
};

use crate::resolver::{
    asn::{
        structs::{
            defs::Asn1ResolvedDefinition,
            types::{
                constructed::{ResolvedComponent, ResolvedConstructedType, ResolvedSeqComponent},
                ioc::{ResolvedFieldSpec, ResolvedObjectSetElement},
                Asn1ResolvedType, ResolvedSetType,
            },
        },
        types::resolve_type,
    },
    Resolver,
};

pub(crate) fn resolve_constructed_type(
    ty: &Asn1Type,
    resolver: &Resolver,
) -> Result<ResolvedConstructedType, Error> {
    if let Asn1TypeKind::Constructed(ref kind) = ty.kind {
        match kind {
            Asn1ConstructedType::Choice(ref c) => resolve_choice_type(c, resolver),
            Asn1ConstructedType::Sequence(ref s) => resolve_sequence_type(s, resolver),
            Asn1ConstructedType::SequenceOf(ref so) => resolve_sequence_of_type(so, resolver),
            _ => {
                eprintln!("ConstructedType: {:#?}", ty);
                Err(resolve_error!("resolve_constructed_Type: Not Implemented!"))
            }
        }
    } else {
        Err(resolve_error!(
            "Expected Constructed Type. Found '{:#?}'",
            ty
        ))
    }
}

fn resolve_choice_type(
    choice: &Asn1TypeChoice,
    resolver: &Resolver,
) -> Result<ResolvedConstructedType, Error> {
    let mut components = vec![];
    for c in &choice.components {
        let ty = resolve_type(&c.ty, resolver)?;
        let component = ResolvedComponent {
            id: c.id.clone(),
            ty,
        };
        components.push(component);
    }

    Ok(ResolvedConstructedType::Choice { components })
}

fn resolve_sequence_type(
    sequence: &Asn1TypeSequence,
    resolver: &Resolver,
) -> Result<ResolvedConstructedType, Error> {
    let mut components = vec![];
    for c in &sequence.root_components {
        let ty = match resolve_type(&c.component.ty, resolver) {
            Ok(ty) => ty,
            Err(_) => {
                let components = sequence
                    .root_components
                    .iter()
                    .map(|c| c.component.clone())
                    .collect::<Vec<Component>>();
                resolve_type_for_components(&c.component.ty, &components, resolver)?
            }
        };
        let component = ResolvedComponent {
            id: c.component.id.clone(),
            ty,
        };
        let seq_component = ResolvedSeqComponent {
            component,
            optional: c.optional,
        };
        components.push(seq_component);
    }

    Ok(ResolvedConstructedType::Sequence { components })
}

fn resolve_sequence_of_type(
    sequence_of: &Asn1TypeSequenceOf,
    resolver: &Resolver,
) -> Result<ResolvedConstructedType, Error> {
    let resolved = resolve_type(&sequence_of.ty, resolver)?;
    Ok(ResolvedConstructedType::SequenceOf {
        ty: Box::new(resolved),
    })
}

fn resolve_type_for_components(
    ty: &Asn1Type,
    _components: &Vec<Component>,
    resolver: &Resolver,
) -> Result<Asn1ResolvedType, Error> {
    if let Asn1TypeKind::Reference(Asn1TypeReference::ClassField { fieldref, .. }) = &ty.kind {
        if ty.constraints.is_none() {
            return Err(resolve_error!(
                "Cannot Resolve ClassRef Type: {:#?} without Table Constraint!",
                ty
            ));
        }
        let constraint = &ty.constraints.as_ref().unwrap()[0];

        let set_reference = constraint.get_set_reference()?;

        let objects = resolver.resolved_defs.get(&set_reference);
        if objects.is_none() {
            return Err(resolve_error!(
                "Object Set '{}' not resolved yet!",
                set_reference
            ));
        }

        let mut types = vec![];
        if let Some(Asn1ResolvedDefinition::ObjectSet(ref set)) = objects {
            for object in &set.objects.elements {
                if let ResolvedObjectSetElement::Object(ref ob) = object {
                    let mut r = ob.fields.iter().filter(|(id, _)| id == &fieldref);
                    let r = r.next();
                    if r.is_some() {
                        match r.unwrap().1 {
                            ResolvedFieldSpec::Type { ty } => {
                                if ty.is_some() {
                                    types.push(ty.as_ref().unwrap().clone());
                                }
                            }
                            ResolvedFieldSpec::FixedTypeValue { typeref, .. } => {
                                types.push(typeref.clone());
                            }
                        }
                        break;
                    }
                }
            }
        }
        Ok(Asn1ResolvedType::Set(ResolvedSetType { types }))
    } else {
        Err(resolve_error!("Expected ClassField Reference!"))
    }
}
