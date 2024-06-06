//! Functionality for handling Resolved ASN.1 ENUMERATED Types

use std::collections::BTreeSet;
use std::convert::TryInto;

use anyhow::Result;

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
    ) -> Result<Asn1ResolvedEnumerated> {
        let mut base = Asn1ResolvedEnumerated {
            extensible: e.ext_marker_index.is_some(),
            ..Default::default()
        };

        // FIXME: TODO Constraints

        let mut root_values = BTreeSet::<i128>::new();
        for v in &e.root_values {
            let named = &v.value;
            if let Some(NamedValue::Number(ref s)) = named {
                let parsed = s.parse::<i128>().unwrap();
                root_values.insert(parsed);
                base.named_root_values.push((v.name.clone(), parsed));
            }
        }

        // TODO: Support all crazy Enumerations.
        if root_values.is_empty() {
            for (value, v) in e.root_values.iter().enumerate() {
                base.named_root_values
                    .push((v.name.clone(), value.try_into().unwrap()));
                root_values.insert(value.try_into().unwrap());
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
            for (value, v) in e.ext_values.iter().enumerate() {
                base.named_ext_values
                    .push((v.name.clone(), value.try_into().unwrap()));
                ext_values.insert(value.try_into().unwrap());
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
