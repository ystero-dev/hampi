use std::collections::BTreeMap;

use crate::error::Error;

use crate::parser::asn::structs::types::{
    constructed::{
        AdditionGroupOrComponent, Asn1TypeChoice, Asn1TypeSequence, Asn1TypeSequenceOf, Component,
    },
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

    eprintln!("sequence: {:#?}", sequence);
    for c in &sequence.root_components {
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

    let additions = if sequence.additions.is_empty() {
        None
    } else {
        let mut additions = vec![];
        for addition in &sequence.additions {
            match addition {
                AdditionGroupOrComponent::AdditionGroup(ref group) => {
                    for c in &group.components {
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
                        additions.push(seq_component);
                    }
                }
                AdditionGroupOrComponent::Component(ref c) => {
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
                    additions.push(seq_component);
                }
            }
        }

        if additions.is_empty() {
            None
        } else {
            Some(additions)
        }
    };

    Ok(Asn1ResolvedType::Constructed(
        ResolvedConstructedType::Sequence {
            components,
            extensible: sequence.extensible,
            name: None,
            additions,
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

    // TODO: Sequence additions as sequence classfield components.

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
                additions: None,
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

#[cfg(test)]
mod tests {

    use super::{resolve_constructed_type, Asn1ResolvedType, ResolvedConstructedType};
    use crate::parser::asn::types::parse_type;
    use crate::resolver::Resolver;
    use crate::tokenizer::tokenize;

    #[test]
    fn e2sm_kpm_with_seq_additions() {
        let measurement_labels = r#" SEQUENCE {
	min						ENUMERATED {true, ...}				OPTIONAL,
	max						ENUMERATED {true, ...}				OPTIONAL,
	avg						ENUMERATED {true, ...}				OPTIONAL,
	...,
	ssbIndex					INTEGER (1.. 65535, ...)			OPTIONAL,
	nonGoB-BFmode-Index	INTEGER (1.. 65535, ...)			OPTIONAL,
	mIMO-mode-Index		INTEGER (1.. 2, ...)					OPTIONAL
}"#;
        let reader = std::io::BufReader::new(std::io::Cursor::new(measurement_labels));
        let tokens = tokenize(reader);
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        let ty = parse_type(&tokens);
        assert!(ty.is_ok(), "{}", ty.err().unwrap());

        let (ty, _) = ty.unwrap();
        let mut resolver = Resolver::new();
        let resolved = resolve_constructed_type(&ty, &mut resolver);
        assert!(resolved.is_ok(), "{}", resolved.err().unwrap());

        let resolved = resolved.unwrap();
        if let Asn1ResolvedType::Constructed(ResolvedConstructedType::Sequence {
            ref components,
            ..
        }) = resolved
        {
            assert!(
                components.len() == 3,
                "expected 3: actual {}",
                components.len()
            );
        };
    }
}
