//! Handling of Resolution of 'base' types.

use std::collections::{BTreeSet, HashMap};

use crate::error::Error;

use crate::parser::asn::structs::types::{
    base::{Asn1TypeBitString, Asn1TypeEnumerated, Asn1TypeInteger, NamedValue},
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
            Asn1BuiltinType::Integer(ref i) => {
                Ok(ResolvedBaseType::Integer(resolve_integer(ty, i, resolver)?))
            }
            Asn1BuiltinType::Enumerated(ref e) => {
                Ok(ResolvedBaseType::Enum(resolve_enumerated(ty, e, resolver)?))
            }
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

// Resolves the parsed integer to it's Resolved variant with right bit width and signedness.
fn resolve_integer(
    ty: &Asn1Type,
    i: &Asn1TypeInteger,
    resolver: &mut Resolver,
) -> Result<Asn1ResolvedInteger, Error> {
    let mut base = Asn1ResolvedInteger::default();

    if ty.constraints.is_none() {
        return Ok(base);
    } else {
        let constraints = ty.constraints.as_ref().unwrap();
        if constraints.is_empty() {
            return Ok(base);
        }
    }

    // Get the Values that are expected
    let value_set = ty.get_integer_valueset_from_constraint(resolver)?;
    if let Some(x) = value_set.root_values.min() {
        if x < 0 {
            base.signed = true
        } else {
            base.signed = false
        }
    }

    let bit_width = if base.signed {
        let min = value_set.root_values.min().unwrap();
        let max = value_set.root_values.max().unwrap();
        let bits_needed_max = 128 - max.abs().leading_zeros();
        let bits_needed_min = 128 - min.abs().leading_zeros();
        std::cmp::max(bits_needed_min, bits_needed_max)
    } else {
        if value_set.root_values.min().is_none() {
            8_u32
        } else {
            let max = value_set.root_values.max().unwrap();
            let bits_needed_max = 128 - max.leading_zeros();
            bits_needed_max
        }
    };

    base.bytes = if bit_width <= 8 {
        1
    } else if bit_width <= 16 {
        2
    } else if bit_width <= 32 {
        4
    } else if bit_width <= 64 {
        8
    } else {
        16
    };

    // TODO: If we have named values, They should be added to Global list of resolved definitions.
    if i.named_values.is_some() {}

    let _ = base.resolved_constraints.replace(value_set);
    Ok(base)
}

fn resolve_enumerated(
    _ty: &Asn1Type,
    e: &Asn1TypeEnumerated,
    _resolver: &Resolver,
) -> Result<Asn1ResolvedEnumerated, Error> {
    let mut base = Asn1ResolvedEnumerated::default();

    // FIXME: TODO Constraints

    let mut values = BTreeSet::<i128>::new();

    // First get all the 'known' values from the Enumerated type into the `values` Set.
    let mut all_values = e.root_values.clone();
    all_values.extend(e.ext_values.clone());
    for v in &all_values {
        let named = &v.value;
        if let Some(NamedValue::Number(ref s)) = named {
            let parsed = s.parse::<i128>().unwrap();
            values.insert(parsed);
        }
    }

    // For all the ASN.1 that we are supporting this is true, so let's just implement this much and
    // go ahead.
    // TODO: Support all crazy Enumerations.
    if values.is_empty() {
        let mut value = 0_i128;
        for v in &all_values {
            base.named_values.insert(v.name.clone(), value);
            values.insert(value);

            value += 1;
        }
    }

    // FIXME: Following is hard-coded
    base.signed = false;
    base.bytes = 1;

    let _ = base.values.replace(values);

    Ok(base)
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
