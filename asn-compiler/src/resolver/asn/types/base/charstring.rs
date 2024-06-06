//! Functionality for handling Resolved ASN.1 CharacterString Types.

use crate::resolver::Resolver;
use anyhow::Result;

use crate::parser::asn::structs::types::{Asn1BuiltinType, Asn1Type, Asn1TypeKind};
use crate::resolver::asn::structs::types::base::Asn1ResolvedCharacterString;

impl Asn1ResolvedCharacterString {
    pub(crate) fn resolve_character_string(
        ty: &Asn1Type,
        resolver: &Resolver,
    ) -> Result<Asn1ResolvedCharacterString> {
        let mut base = Asn1ResolvedCharacterString::default();

        if let Asn1TypeKind::Builtin(Asn1BuiltinType::CharacterString { str_type }) = &ty.kind {
            base.str_type = str_type.clone();
        }
        if ty.constraints.is_some() {
            let constraints = ty.constraints.as_ref().unwrap();
            if !constraints.is_empty() {
                let constraint = &constraints[0];

                if constraint.is_size_constraint() {
                    let value_set = constraint.get_size_valueset(resolver)?;
                    let _ = base.size.replace(value_set);
                } else {
                    log::warn!(
                        "Found Constraint '{:#?}'. Only supported constraint on CharacterString Type is Size Constraint.",
                        constraint
                    );
                }
            }
        }
        Ok(base)
    }
}
