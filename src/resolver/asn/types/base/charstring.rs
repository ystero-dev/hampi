//! Functionality for handling Resolved ASN.1 CharacterString Types.

use crate::error::Error;
use crate::resolver::Resolver;

use crate::parser::asn::structs::types::Asn1Type;
use crate::resolver::asn::structs::types::base::Asn1ResolvedCharacterString;

impl Asn1ResolvedCharacterString {
    pub(crate) fn resolve_character_string(
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
}
