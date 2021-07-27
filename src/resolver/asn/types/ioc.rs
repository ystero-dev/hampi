//! Handling of Information Object Classes, ObjectSets, Objects etc

use crate::error::Error;

use crate::parser::asn::structs::types::ioc::{Asn1Object, Asn1ObjectSet, ObjectSetElement};

use crate::resolver::{
    asn::structs::{
        defs::Asn1ResolvedDefinition,
        types::ioc::{
            Asn1ResolvedObject, Asn1ResolvedObjectSet, ResolvedObjectSet, ResolvedObjectSetElement,
        },
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
            let element = match object {
                ObjectSetElement::ObjectSetReference(ref r)
                | ObjectSetElement::ObjectReference(ref r) => {
                    let resolved = resolver.resolved_defs.get(r);
                    if resolved.is_none() {
                        return Err(resolve_error!(
                            "Unable to find the Referencing '{}' Object Set while resolving {:#?}",
                            r,
                            object
                        ));
                    } else {
                        let resolved = resolved.unwrap();
                        let element = match resolved {
                            Asn1ResolvedDefinition::ObjectSet(..) => {
                                ResolvedObjectSetElement::ObjectSetReference(r.clone())
                            }
                            Asn1ResolvedDefinition::Object(..) => {
                                ResolvedObjectSetElement::ObjectReference(r.clone())
                            }
                            _ => {
                                return Err(resolve_error!(
                                    "Matching definition found for reference '{}' but is a '{:#?}' and not an ObjectSet or an Object!",
                                    r,
                                    resolved,
                                ));
                            }
                        };
                        element
                    }
                }
                ObjectSetElement::Object(..) => {
                    ResolvedObjectSetElement::Object(Asn1ResolvedObject::default())
                }
            };
            elements.push(element);
        }
        Ok(Asn1ResolvedObjectSet {
            objects: ResolvedObjectSet { elements },
        })
    }
}

pub(crate) fn resolve_object(
    object: &Asn1Object,
    resolver: &Resolver,
) -> Result<Asn1ResolvedObject, Error> {
    let class = resolver.classes.get(&object.class);
    if class.is_none() {
        Err(resolve_error!(
            "Class '{}' definition not found to resolve object!",
            object.class
        ))
    } else {
        Ok(Asn1ResolvedObject::default())
    }
}
