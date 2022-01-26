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
            values::Asn1ResolvedValue,
        },
        types::resolve_type,
        values::resolve_value,
    },
    Resolver,
};

pub(crate) fn resolve_object_set(
    objectset: &Asn1ObjectSet,
    name: &str,
    resolver: &mut Resolver,
) -> Result<Asn1ResolvedObjectSet, Error> {
    let class = resolver.classes.get(&objectset.class);
    match class {
        None => Err(resolve_error!(
            "Class '{}' definition not found to resolve object set!",
            objectset.class
        )),
        Some(cls) => {
            // if the class has UNIQUE field, create a Map for the values from that field to the
            // Object. Note: it's indeed possible to have more than one unique field, but we are not
            // supporting it at the moment, we'll create the map for the 'first' unique field.
            let class = cls.get_inner_class().unwrap().classref; // inner class is guaranteed to be Some or a BUG, Let it Panic
            let unique_field = class.get_first_unique_field_id();
            let unique_field = if let Some(u) = unique_field {
                u
            } else {
                "".to_string()
            };
            let mut lookup_table = HashMap::new();

            let mut elements = vec![];
            let mut input_elements = objectset.objects.root_elements.clone();
            input_elements.extend(objectset.objects.additional_elements.clone());
            let mut decoder_ty = None;
            for (i, object) in input_elements.iter().enumerate() {
                match object {
                    ObjectSetElement::ObjectSetReference(ref r) => {
                        let resolved = resolver.resolved_defs.get(r);
                        match resolved {
                            None => {
                                return Err(resolve_error!(
                            "Unable to find the Referencing '{}' Object Set while resolving {:#?}",
                            r,
                            objectset
                        ));
                            }
                            Some(res) => {
                                if let Asn1ResolvedDefinition::ObjectSet(ref o) = res {
                                    decoder_ty = o.objects.decoder_ty.clone();
                                    elements.extend(o.objects.elements.clone());
                                    lookup_table.extend(o.objects.lookup_table.clone());
                                } else {
                                    return Err(resolve_error!(
                                        "Resolved '{}' is not an Object Set!",
                                        r,
                                    ));
                                }
                            }
                        }
                    }
                    ObjectSetElement::ObjectReference(ref r) => {
                        let resolved = resolver.resolved_defs.get(r);
                        match resolved {
                            None => {
                                return Err(resolve_error!(
                            "Unable to find the Referencing '{}' Object Set while resolving {:#?}",
                            r,
                            objectset
                        ));
                            }
                            Some(res) => {
                                if let Asn1ResolvedDefinition::Object(ref o) = res {
                                    let element =
                                        ResolvedObjectSetElement::Object(Asn1ResolvedObject {
                                            name: r.clone(),
                                            fields: o.fields.clone(),
                                        });
                                    elements.push(element.clone());
                                    if !unique_field.is_empty() {
                                        if let ResolvedObjectSetElement::Object(ref o) = element {
                                            let field = o.fields.get(&unique_field);
                                            if let ResolvedFieldSpec::FixedTypeValue {
                                                value,
                                                typeref,
                                            } = field.unwrap()
                                            {
                                                decoder_ty.replace(typeref.clone());
                                                let value = value.as_ref().unwrap();
                                                if let Asn1ResolvedValue::Reference(ref s) = value {
                                                    let v = resolver.resolved_defs.get(s).unwrap();

                                                    if let Asn1ResolvedDefinition::Value(ref v) = v
                                                    {
                                                        // Get the actual Value - Usually this value is an
                                                        // Integer Value. This Value is to be used by the
                                                        // decoder.
                                                        let v = format!(
                                                            "{}",
                                                            v.get_base_integer_value().unwrap()
                                                        );
                                                        lookup_table
                                                            .insert((v, s.clone()), element);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    return Err(resolve_error!(
                                        "Resolved '{}' is not an Object!",
                                        r,
                                    ));
                                }
                            }
                        }
                    }
                    ObjectSetElement::Object(ref v) => {
                        let object_name = format!("{}{}", name, i);
                        let element = ResolvedObjectSetElement::Object(resolve_object(
                            &object_name,
                            v,
                            resolver,
                        )?);
                        elements.push(element.clone());
                        if !unique_field.is_empty() {
                            if let ResolvedObjectSetElement::Object(ref o) = element {
                                let field = o.fields.get(&unique_field);
                                if let ResolvedFieldSpec::FixedTypeValue { value, typeref } =
                                    field.unwrap()
                                {
                                    decoder_ty.replace(typeref.clone());
                                    let value = value.as_ref().unwrap();
                                    if let Asn1ResolvedValue::Reference(ref s) = value {
                                        let v = resolver.resolved_defs.get(s).unwrap();

                                        if let Asn1ResolvedDefinition::Value(ref v) = v {
                                            // Get the actual Value - Usually this value is an
                                            // Integer Value. This Value is to be used by the
                                            // decoder.
                                            let v =
                                                format!("{}", v.get_base_integer_value().unwrap());
                                            lookup_table.insert((v, s.clone()), element);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Ok(Asn1ResolvedObjectSet {
                objects: ResolvedObjectSet {
                    decoder_ty,
                    elements,
                    lookup_table,
                },
            })
        }
    }
}

pub(crate) fn resolve_object(
    object_name: &str,
    object_value: &Asn1ObjectValue,
    resolver: &mut Resolver,
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
            name: object_name.to_string(),
            fields: resolved_fields,
        })
    } else {
        Err(resolve_error!(
            "Unsupported Variant while Resolving {:#?}",
            object_value
        ))
    }
}
