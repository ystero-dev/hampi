//! Functionality for handling Resolved ASN.1 Enumerated Types
use std::collections::BTreeSet;

use crate::error::Error;

use crate::parser::asn::structs::types::{
    base::{Asn1TypeEnumerated, NamedValue},
    Asn1Type,
};
use crate::resolver::asn::structs::types::base::Asn1ResolvedEnumerated;
use crate::resolver::Resolver;

impl Asn1ResolvedEnumerated {
    pub(super) fn resolve_enumerated(
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
        base.bits = 8;

        let _ = base.values.replace(values);

        Ok(base)
    }
}
