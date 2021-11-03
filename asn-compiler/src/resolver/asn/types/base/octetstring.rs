//! Functionality for handling Resolved ASN.1 OCTET STRING

use crate::error::Error;
use crate::resolver::Resolver;

use crate::parser::asn::structs::types::Asn1Type;
use crate::resolver::asn::structs::types::base::Asn1ResolvedOctetString;

impl Asn1ResolvedOctetString {
    pub(crate) fn resolve_octet_string(
        ty: &Asn1Type,
        resolver: &Resolver,
    ) -> Result<Asn1ResolvedOctetString, Error> {
        let mut base = Asn1ResolvedOctetString::default();

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
        Ok(base)
    }
}
