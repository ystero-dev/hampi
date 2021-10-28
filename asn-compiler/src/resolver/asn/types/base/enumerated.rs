//! Functionality for handling Resolved ASN.1 ENUMERATED Types

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

        base.extensible = e.ext_marker_index.is_some();

        // FIXME: TODO Constraints

        let mut root_values = BTreeSet::<i128>::new();
        for v in &e.root_values {
            let named = &v.value;
            if let Some(NamedValue::Number(ref s)) = named {
                let parsed = s.parse::<i128>().unwrap();
                root_values.insert(parsed);
            }
        }
        // TODO: Support all crazy Enumerations.
        if root_values.is_empty() {
            let mut value = 0_i128;
            for v in &e.root_values {
                base.named_root_values.push((v.name.clone(), value));
                root_values.insert(value);

                value += 1;
            }
        }

        let mut ext_values = BTreeSet::<i128>::new();
        for v in &e.ext_values {
            let named = &v.value;
            if let Some(NamedValue::Number(ref s)) = named {
                let parsed = s.parse::<i128>().unwrap();
                ext_values.insert(parsed);
            }
        }
        // TODO: Support all crazy Enumerations.
        if ext_values.is_empty() {
            let mut value = 0_i128;
            for v in &e.ext_values {
                base.named_ext_values.push((v.name.clone(), value));
                ext_values.insert(value);

                value += 1;
            }
        }

        // FIXME: Following is hard-coded
        base.signed = false;
        base.bits = 8;

        let _ = base.root_values.replace(root_values);
        let _ = base.ext_values.replace(ext_values);

        Ok(base)
    }
}
