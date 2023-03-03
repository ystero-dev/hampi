use std::collections::BTreeMap;

use crate::error::Error;

use crate::parser::asn::structs::types::{
    constructed::{Asn1TypeChoice, Asn1TypeSequence, Asn1TypeSequenceOf, Component, SeqComponent},
    Asn1ConstructedType, Asn1Type, Asn1TypeKind, Asn1TypeReference,
};

use crate::resolver::{
    asn::{
        structs::{
            defs::Asn1ResolvedDefinition,
            types::{
                constructed::{
                    ClassFieldComponentType, ResolvedComponent, ResolvedConstructedType,
                    ResolvedSeqComponent,
                },
                ioc::{ResolvedFieldSpec, ResolvedObjectSet, ResolvedObjectSetElement},
                Asn1ResolvedType, ResolvedSetType, ResolvedSetTypeMap,
            },
        },
        types::resolve_type,
    },
    Resolver,
};

pub(crate) fn resolve_constructed_type(
    ty: &Asn1Type,
    resolver: &mut Resolver,
) -> Result<Asn1ResolvedType, Error> {
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
    resolver: &mut Resolver,
) -> Result<Asn1ResolvedType, Error> {
    let mut root_components = vec![];
    for c in &choice.root_components {
        let ty = resolve_type(&c.ty, resolver)?;
        let component = ResolvedComponent {
            id: c.id.clone(),
            ty,
        };
        root_components.push(component);
    }

    let additions = if choice.additions.is_some() {
        let mut components = vec![];
        for addition in choice.additions.as_ref().unwrap() {
            for c in &addition.components {
                let ty = resolve_type(&c.ty, resolver)?;
                let component = ResolvedComponent {
                    id: c.id.clone(),
                    ty,
                };
                components.push(component);
            }
        }
        Some(components)
    } else {
        None
    };

    Ok(Asn1ResolvedType::Constructed(
        ResolvedConstructedType::Choice {
            name: None,
            root_components,
            additions,
        },
    ))
}

fn resolve_sequence_type(
    sequence: &Asn1TypeSequence,
    resolver: &mut Resolver,
) -> Result<Asn1ResolvedType, Error> {
    let mut components = vec![];
    // FIXME: implement for additional_components too
    let mut all_components = sequence.root_components.clone();
    for addition in &sequence.additions {
        all_components.extend(addition.components.clone());
    }
    for c in all_components {
        let ty = match resolve_type(&c.component.ty, resolver) {
            Ok(ty) => ty,
            Err(_e) => {
                return resolve_sequence_classfield_components(sequence, resolver);
            }
        };
        let component = ResolvedComponent {
            id: c.component.id.clone(),
            ty,
        };
        let seq_component = ResolvedSeqComponent {
            component,
            optional: c.optional || c.default.is_some(),
            class_field_type: None,
            key_field: false,
        };
        components.push(seq_component);
    }

    Ok(Asn1ResolvedType::Constructed(
        ResolvedConstructedType::Sequence {
            components,
            extensible: sequence.extensible,
            name: None,
        },
    ))
}

fn resolve_sequence_of_type(
    sequence_of: &Asn1TypeSequenceOf,
    resolver: &mut Resolver,
) -> Result<Asn1ResolvedType, Error> {
    let resolved = resolve_type(&sequence_of.ty, resolver)?;
    let size_values = if sequence_of.size.is_some() {
        let size = sequence_of.size.as_ref().unwrap();
        Some(size.get_size_valueset(resolver)?)
    } else {
        None
    };

    Ok(Asn1ResolvedType::Constructed(
        ResolvedConstructedType::SequenceOf {
            ty: Box::new(resolved),
            name: None,
            size_values,
        },
    ))
}

fn resolve_sequence_classfield_components(
    seq: &Asn1TypeSequence,
    resolver: &mut Resolver,
) -> Result<Asn1ResolvedType, Error> {
    let mut all_components = vec![];
    all_components.extend(seq.root_components.clone());
    all_components.extend(
        seq.additions
            .clone()
            .iter()
            .flat_map(|a| a.components.clone())
            .collect::<Vec<SeqComponent>>(),
    );
    let all_components = all_components
        .iter()
        .map(|c| c.component.clone())
        .collect::<Vec<Component>>();

    if all_components.is_empty() {
        // It's an Error to try to resolve Empty components with Class Field Ref
        return Err(resolve_error!("Expected Sequence with at-least one ClassField Reference Component!. Found Empty Sequences"));
    }

    // Get the Object Set first It's the Same Object Set for all components. So if we get the first
    // one that's good enough!
    let ty = &all_components[0].ty;
    let constraint = &ty.constraints.as_ref().unwrap()[0];

    let set_reference = constraint.get_set_reference()?;

    let objects = resolver.resolved_defs.get(&set_reference);
    if objects.is_none() {
        return Err(resolve_error!(
            "Object Set '{}' not resolved yet!",
            set_reference
        ));
    }
    let objects = objects.unwrap().clone();

    if let Asn1ResolvedDefinition::ObjectSet(ref set) = objects {
        let objects = &set.objects;
        let components =
            resolve_seq_components_for_objects(&all_components, &set_reference, objects, resolver)?;
        Ok(Asn1ResolvedType::Constructed(
            ResolvedConstructedType::Sequence {
                name: None,
                extensible: seq.extensible,
                components,
            },
        ))
    } else {
        Err(resolve_error!(
            "Object Set '{}' not resolved yet!",
            set_reference
        ))
    }
}

fn resolve_seq_components_for_objects(
    input_components: &[Component],
    set_reference: &str,
    objects: &ResolvedObjectSet,
    resolver: &mut Resolver,
) -> Result<Vec<ResolvedSeqComponent>, Error> {
    if objects.elements.is_empty() {
        return Ok(vec![]);
    }
    let first = &objects.elements[0];
    let mut result = vec![];
    for component in input_components {
        if let Asn1TypeKind::Reference(Asn1TypeReference::ClassField { fieldref, .. }) =
            &component.ty.kind
        {
            if let ResolvedObjectSetElement::Object(ref o) = first {
                let spec = o.fields.get(fieldref);
                if let Some(ResolvedFieldSpec::FixedTypeValue { typeref, .. }) = spec {
                    let constraint = &component.ty.constraints.as_ref().unwrap()[0];
                    let comp_spec = constraint.get_comp_reference();
                    let component = ResolvedComponent {
                        id: component.id.clone(),
                        ty: typeref.clone(),
                    };
                    let seq_component = ResolvedSeqComponent {
                        component,
                        optional: false, // FIXME:
                        class_field_type: Some(ClassFieldComponentType::FixedTypeValue),
                        key_field: comp_spec.is_none(),
                    };
                    result.push(seq_component);
                } else {
                    let types = get_seq_component_for_object_set(fieldref, objects)?;
                    let ty = ResolvedSetType {
                        setref: set_reference.to_string(),
                        types,
                    };
                    let component = ResolvedComponent {
                        id: component.id.clone(),
                        ty: Asn1ResolvedType::Set(ty),
                    };
                    let seq_component = ResolvedSeqComponent {
                        component,
                        optional: false, // FIXME:
                        class_field_type: Some(ClassFieldComponentType::Type),
                        key_field: false,
                    };
                    result.push(seq_component);
                }
            } else {
                log::warn!("WARN!");
            }
        } else {
            // It is possible that there are types in an Object Set that are not Class Reference,
            // we simply try to `resolve_type` them like sequence above, except an error here is an
            // actual error, since we are not expecting ClassField components inside Classfield
            // components (For now).
            let ty = resolve_type(&component.ty, resolver)?;
            let component = ResolvedComponent {
                id: component.id.clone(),
                ty,
            };
            let seq_component = ResolvedSeqComponent {
                component,
                optional: false, // FIXME: Not sure
                class_field_type: None,
                key_field: false,
            };
            result.push(seq_component);
        }
    }
    // Only add to the result if Every Component in the Input is also found in the Object. This
    // is mainly true for optional components. If Optional Components are missing, They should
    // not be part of the generated Set.
    Ok(result)
}

fn get_seq_component_for_object_set(
    fieldref: &str,
    objects: &ResolvedObjectSet,
) -> Result<ResolvedSetTypeMap, Error> {
    let mut types = BTreeMap::new();
    for (key, object) in &objects.lookup_table {
        if let ResolvedObjectSetElement::Object(ref o) = object {
            let field_ob = o.fields.get(fieldref);
            if let Some(ResolvedFieldSpec::Type { ty }) = field_ob {
                if let Some(Asn1ResolvedType::Reference(ref _tyid)) = ty {
                    types.insert(
                        (key.1.clone(), key.0.clone()),
                        (key.0.clone(), ty.as_ref().unwrap().clone()),
                    );
                }
            }
        }
    }
    Ok(types)
}
