//! Functionality for handling Resolved ASN.1 INTEGER Types

use crate::error::Error;

use crate::parser::asn::structs::types::{base::Asn1TypeInteger, Asn1Type};
use crate::resolver::asn::structs::types::base::Asn1ResolvedInteger;
use crate::resolver::Resolver;

impl Asn1ResolvedInteger {
    // Resolves the parsed integer to it's Resolved variant with right bit width and signedness.
    pub(super) fn resolve_integer(
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

        base.bits = if bit_width <= 8 {
            8
        } else if bit_width <= 16 {
            16
        } else if bit_width <= 32 {
            32
        } else if bit_width <= 64 {
            64
        } else {
            128
        };

        // TODO: If we have named values, They should be added to Global list of resolved definitions.
        if i.named_values.is_some() {}

        let _ = base.resolved_constraints.replace(value_set);
        Ok(base)
    }
}
