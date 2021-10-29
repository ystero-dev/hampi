//! Functionality for handling Resolved ASN.1 BIT STRING

use std::collections::HashMap;

use crate::error::Error;

use crate::parser::asn::structs::types::{
    base::{Asn1TypeBitString, NamedValue},
    Asn1Type,
};
use crate::resolver::asn::structs::types::base::Asn1ResolvedBitString;
use crate::resolver::Resolver;

impl Asn1ResolvedBitString {
    pub(crate) fn resolve_bit_string(
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
                    let value_set = constraint.get_size_valueset(resolver)?;
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
}
