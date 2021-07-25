//! Handling of Information Object Classes, ObjectSets, Objects etc

use crate::error::Error;
use crate::structs::parser::types::ioc::{Asn1Object, Asn1ObjectSet};
use crate::structs::resolver::{
    types::ioc::{Asn1ResolvedObject, Asn1ResolvedObjectSet},
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
        Ok(Asn1ResolvedObjectSet::default())
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
