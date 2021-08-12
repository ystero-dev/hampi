//! Handling of Resolution of 'base' types.

mod integer;

mod enumerated;

use std::collections::HashMap;

use crate::error::Error;

use crate::parser::asn::structs::types::{
    base::{Asn1TypeBitString, NamedValue},
    Asn1BuiltinType, Asn1Type, Asn1TypeKind, Asn1TypeReference,
};

use crate::resolver::{
    asn::structs::types::{
        base::{
            Asn1ResolvedBitString, Asn1ResolvedBoolean, Asn1ResolvedCharacterString,
            Asn1ResolvedEnumerated, Asn1ResolvedInteger, Asn1ResolvedObjectIdentifier,
            Asn1ResolvedOctetString, ResolvedBaseType,
        },
        constraints::Asn1ConstraintValueSet,
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
    ) -> Result<Asn1ConstraintValueSet, Error> {
        let kind = &self.kind;
        match kind {
            Asn1TypeKind::Builtin(Asn1BuiltinType::Integer(..)) => {
                let constraint = &self.constraints.as_ref().unwrap()[0];
                constraint.get_integer_valueset(resolver)
            }
            Asn1TypeKind::Reference(Asn1TypeReference::Reference(ref _r)) => {
                Err(constraint_error!("Not Implemented!"))
            }
            _ => Err(constraint_error!(
                "The Type '{:#?}' is not of a BuiltIn Or a Referenced Kind!",
                self,
            )),
        }
    }
}

pub(crate) fn resolve_base_type(
    ty: &Asn1Type,
    resolver: &mut Resolver,
) -> Result<ResolvedBaseType, Error> {
    if let Asn1TypeKind::Builtin(ref kind) = ty.kind {
        match kind {
            Asn1BuiltinType::Integer(ref i) => Ok(ResolvedBaseType::Integer(
                Asn1ResolvedInteger::resolve_integer(ty, i, resolver)?,
            )),
            Asn1BuiltinType::Enumerated(ref e) => Ok(ResolvedBaseType::Enum(
                Asn1ResolvedEnumerated::resolve_enumerated(ty, e, resolver)?,
            )),
            Asn1BuiltinType::BitString(ref b) => Ok(ResolvedBaseType::BitString(
                resolve_bit_string(ty, b, resolver)?,
            )),
            Asn1BuiltinType::Boolean => {
                let resolved = ResolvedBaseType::Boolean(Asn1ResolvedBoolean::default());
                Ok(resolved)
            }
            Asn1BuiltinType::OctetString => Ok(ResolvedBaseType::OctetString(
                resolve_octet_string(ty, resolver)?,
            )),
            Asn1BuiltinType::CharacterString => Ok(ResolvedBaseType::CharacterString(
                resolve_character_string(ty, resolver)?,
            )),
            Asn1BuiltinType::ObjectIdentifier => {
                let resolved =
                    ResolvedBaseType::ObjectIdentifier(Asn1ResolvedObjectIdentifier::default());
                Ok(resolved)
            }
            _ => Err(resolve_error!(
                "parse_base_type: Not Implemented! {:#?}",
                ty
            )),
        }
    } else {
        Err(resolve_error!("Expected Base Type. Found '{:#?}'", ty))
    }
}

fn resolve_character_string(
    ty: &Asn1Type,
    resolver: &Resolver,
) -> Result<Asn1ResolvedCharacterString, Error> {
    let mut base = Asn1ResolvedCharacterString::default();

    if ty.constraints.is_some() {
        let constraints = ty.constraints.as_ref().unwrap();
        if !constraints.is_empty() {
            let constraint = &constraints[0];

            if constraint.is_size_constraint() {
                let value_set = constraint.get_integer_valueset(resolver)?;
                let _ = base.size.replace(value_set);
            } else {
                return Err(constraint_error!(
                    "Expected a Size Constraint, Found '{:#?}'",
                    constraint
                ));
            }
        }
    }
    Ok(base)
}

fn resolve_octet_string(
    ty: &Asn1Type,
    resolver: &Resolver,
) -> Result<Asn1ResolvedOctetString, Error> {
    let mut base = Asn1ResolvedOctetString::default();

    if ty.constraints.is_some() {
        let constraints = ty.constraints.as_ref().unwrap();
        if !constraints.is_empty() {
            let constraint = &constraints[0];

            if constraint.is_size_constraint() {
                let value_set = constraint.get_integer_valueset(resolver)?;
                let _ = base.size.replace(value_set);
            } else {
                // The constraint is a 'CONTAINING' Constraint, TODO: Handle this.
            }
        }
    }
    Ok(base)
}

fn resolve_bit_string(
    ty: &Asn1Type,
    b: &Asn1TypeBitString,
    resolver: &Resolver,
) -> Result<Asn1ResolvedBitString, Error> {
    let mut base = Asn1ResolvedBitString::default();

    if ty.constraints.is_some() {
        let constraints = ty.constraints.as_ref().unwrap();
        if !constraints.is_empty() {
            let constraint = &constraints[0];

            if constraint.is_size_constraint() {
                let value_set = constraint.get_integer_valueset(resolver)?;
                let _ = base.size.replace(value_set);
            } else {
                // The constraint is a 'CONTAINING' Constraint, TODO: Handle this.
            }
        }
    }

    if b.named_bits.is_some() {
        let mut named_values = HashMap::new();
        for nb in b.named_bits.as_ref().unwrap() {
            if let NamedValue::Number(ref v) = nb.1 {
                let parsed = v.parse::<u8>().unwrap(); // Let it panic if it does not fit in u8.
                named_values.insert(nb.0.clone(), parsed);
            }
        }
        base.named_values.extend(named_values);
    }
    Ok(base)
}
