//! Handling of Information Object Classes, ObjectSets, Objects etc
use std::collections::HashMap;

use crate::error::Error;

use crate::parser::asn::structs::types::ioc::{
    Asn1ObjectFieldSpec, Asn1ObjectSet, Asn1ObjectValue, ObjectSetElement,
};

use crate::resolver::{
    asn::{
        structs::{
            defs::Asn1ResolvedDefinition,
            types::ioc::{
                Asn1ResolvedObject, Asn1ResolvedObjectSet, ResolvedFieldSpec, ResolvedObjectSet,
                ResolvedObjectSetElement,
            },
        },
        types::resolve_type,
        values::resolve_value,
    },
    Resolver,
};

pub(crate) fn resolve_object_set(
    objectset: &Asn1ObjectSet,
    resolver: &Resolver,
) -> Result<Asn1ResolvedObjectSet, Error> {
    let class = resolver.classes.get(&objectset.class);
    if class.is_none() {
        Err(resolve_error!(
            "Class '{}' definition not found to resolve object set!",
            objectset.class
        ))
    } else {
        let mut elements = vec![];
        for object in &objectset.objects.root_elements {
            match object {
                ObjectSetElement::ObjectSetReference(ref r) => {
                    let resolved = resolver.resolved_defs.get(r);
                    if resolved.is_none() {
                        return Err(resolve_error!(
                            "Unable to find the Referencing '{}' Object Set while resolving {:#?}",
                            r,
                            objectset
                        ));
                    } else {
                        if let Asn1ResolvedDefinition::ObjectSet(ref o) = resolved.unwrap() {
                            elements.extend(o.objects.elements.clone());
                        } else {
                            return Err(resolve_error!("Resolved '{}' is not an Object Set!", r,));
                        }
                    }
                }
                ObjectSetElement::ObjectReference(ref r) => {
                    let resolved = resolver.resolved_defs.get(r);
                    if resolved.is_none() {
                        return Err(resolve_error!(
                            "Unable to find the Referencing '{}' Object Set while resolving {:#?}",
                            r,
                            objectset
                        ));
                    } else {
                        if let Asn1ResolvedDefinition::Object(ref o) = resolved.unwrap() {
                            let element = ResolvedObjectSetElement::Object(Asn1ResolvedObject {
                                fields: o.fields.clone(),
                            });
                            elements.push(element);
                        } else {
                            return Err(resolve_error!("Resolved '{}' is not an Object!", r,));
                        }
                    }
                }
                ObjectSetElement::Object(ref v) => {
                    let element = ResolvedObjectSetElement::Object(resolve_object(v, resolver)?);
                    elements.push(element);
                }
            }
        }
        Ok(Asn1ResolvedObjectSet {
            objects: ResolvedObjectSet { elements },
        })
    }
}

pub(crate) fn resolve_object(
    object_value: &Asn1ObjectValue,
    resolver: &Resolver,
) -> Result<Asn1ResolvedObject, Error> {
    let mut resolved_fields = HashMap::new();
    if let Asn1ObjectValue::Asn1ObjectFromClass { fields } = object_value {
        for (k, field) in fields {
            let resolved = match field {
                Asn1ObjectFieldSpec::Type { ty } => {
                    let resolved = if ty.is_some() {
                        Some(resolve_type(&ty.as_ref().unwrap(), resolver)?)
                    } else {
                        None
                    };
                    ResolvedFieldSpec::Type { ty: resolved }
                }
                Asn1ObjectFieldSpec::FixedTypeValue { typeref, value } => {
                    let resolved_type = resolve_type(typeref, resolver)?;

                    let resolved_value = if value.is_some() {
                        Some(resolve_value(
                            &value.as_ref().unwrap(),
                            &resolved_type,
                            resolver,
                        )?)
                    } else {
                        None
                    };
                    ResolvedFieldSpec::FixedTypeValue {
                        typeref: resolved_type,
                        value: resolved_value,
                    }
                }
            };
            resolved_fields.insert(k.clone(), resolved);
        }
        Ok(Asn1ResolvedObject {
            fields: resolved_fields,
        })
    } else {
        Err(resolve_error!(
            "Unsupported Variant while Resolving {:#?}",
            object_value
        ))
    }
}
