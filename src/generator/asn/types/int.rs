#![allow(dead_code)]
//! Handling top level Generator code for a Resolved Type. Based on the individual type variant,
//! the respective functions are called.

use proc_macro2::{Ident, TokenStream};

use crate::error::Error;
use crate::generator::Generator;
use crate::resolver::asn::structs::types::{Asn1ResolvedType, ResolvedSetType};

impl Asn1ResolvedType {
    pub(crate) fn generate_for_type(
        name: &str,
        ty: &Asn1ResolvedType,
        gen: &mut Generator,
    ) -> Result<Option<TokenStream>, Error> {
        match ty {
            Asn1ResolvedType::Base(ref b) => Ok(Some(b.generate_for_base_type(name, gen)?)),
            _ => Ok(None),
        }
    }
    pub(crate) fn generate_name_maybe_aux_type(
        ty: &Asn1ResolvedType,
        generator: &mut Generator,
    ) -> Result<Ident, Error> {
        match ty {
            Asn1ResolvedType::Base(ref b) => b.generate_ident_and_aux_type_for_base(generator),
            Asn1ResolvedType::Reference(ref r) => {
                Asn1ResolvedType::generate_ident_for_reference(r, generator)
            }
            Asn1ResolvedType::Constructed(ref c) => {
                c.generate_ident_and_aux_type_for_constucted(generator)
            }
            Asn1ResolvedType::Set(ref s) => s.generate_ident_and_aux_types_for_set(generator),
        }
    }

    pub(crate) fn generate_ident_for_reference(
        reference: &str,
        gen: &mut Generator,
    ) -> Result<Ident, Error> {
        Ok(gen.to_type_ident(reference))
    }
}

impl ResolvedSetType {
    pub(crate) fn generate_ident_and_aux_types_for_set(
        &self,
        _gen: &mut Generator,
    ) -> Result<Ident, Error> {
        Err(resolve_error!("Not Implemented!"))
    }
}
