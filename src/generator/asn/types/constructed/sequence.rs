#![allow(dead_code)]
//! Code Generator for the SEQUENCE Type.

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::error::Error;
use crate::generator::Generator;
use crate::resolver::asn::structs::types::{
    constructed::ResolvedConstructedType, Asn1ResolvedType,
};

impl ResolvedConstructedType {
    pub(crate) fn generate_ident_and_aux_type_for_constucted(
        &self,
        generator: &mut Generator,
    ) -> Result<Ident, Error> {
        let unique_name = generator.to_unique_name("CONSTRUCTED");
        let generated_type = self.generate(&unique_name, generator)?;
        generator.aux_items.push(generated_type);
        Ok(generator.to_type_ident(&unique_name))
    }

    pub(crate) fn generate(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        match self {
            ResolvedConstructedType::Sequence { .. } => self.generate_sequence(name, generator),
            ResolvedConstructedType::Choice { .. } => self.generate_choice(name, generator),
            ResolvedConstructedType::SequenceOf { .. } => Ok(TokenStream::new()),
        }
    }

    pub(crate) fn generate_sequence(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        if let ResolvedConstructedType::Sequence { ref components } = self {
            let type_name = generator.to_type_ident(name);
            let mut comp_tokens = TokenStream::new();
            for c in components {
                let comp_field_ident = generator.to_value_ident(&c.component.id);
                let comp_ty_ident =
                    Asn1ResolvedType::generate_name_maybe_aux_type(&c.component.ty, generator)?;
                let comp_token = quote! {
                    pub #comp_field_ident: #comp_ty_ident,
                };
                comp_tokens.extend(comp_token);
            }
            Ok(quote! {
                pub struct #type_name {
                    #comp_tokens
                }
            })
        } else {
            Ok(TokenStream::new())
        }
    }

    pub(crate) fn generate_choice(
        &self,
        _name: &str,
        _generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        Ok(TokenStream::new())
    }
}
