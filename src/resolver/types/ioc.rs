//! Handling of Information Object Classes, ObjectSets, Objects etc

use std::collections::HashMap;

use crate::error::Error;
use crate::structs::parser::defs::{Asn1Definition, Asn1ObjectClassAssignment};
use crate::structs::parser::types::ioc::{Asn1Object, Asn1ObjectSet};
use crate::structs::resolver::defs::Asn1ResolvedDefinition;
use crate::structs::resolver::types::ioc::{Asn1ResolvedObject, Asn1ResolvedObjectSet};

pub(crate) fn resolve_object_set(
    objectset: &Asn1ObjectSet,
    _resolved_defs: &HashMap<String, Asn1ResolvedDefinition>,
    _parameterized_defs: &HashMap<String, Asn1Definition>,
    object_classes: &HashMap<String, Asn1ObjectClassAssignment>,
) -> Result<Asn1ResolvedObjectSet, Error> {
    let class = object_classes.get(&objectset.class);
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
    _resolved_defs: &HashMap<String, Asn1ResolvedDefinition>,
    _parameterized_defs: &HashMap<String, Asn1Definition>,
    object_classes: &HashMap<String, Asn1ObjectClassAssignment>,
) -> Result<Asn1ResolvedObject, Error> {
    let class = object_classes.get(&object.class);
    if class.is_none() {
        Err(resolve_error!(
            "Class '{}' definition not found to resolve object!",
            object.class
        ))
    } else {
        Ok(Asn1ResolvedObject::default())
    }
}
