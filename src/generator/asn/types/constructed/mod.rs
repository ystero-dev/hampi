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
        let unique_name = match self {
            ResolvedConstructedType::Sequence { name, .. } => {
                if name.is_some() {
                    name.as_ref().unwrap().clone()
                } else {
                    generator.to_unique_name("Sequence")
                }
            }
            ResolvedConstructedType::Choice { .. } => generator.to_unique_name("Choice"),
            ResolvedConstructedType::SequenceOf { .. } => generator.to_unique_name("SeqOf"),
        };

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
            ResolvedConstructedType::SequenceOf { .. } => {
                self.generate_sequence_of(name, generator)
            }
        }
    }

    pub(crate) fn generate_sequence(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        if let ResolvedConstructedType::Sequence { ref components, .. } = self {
            let type_name = generator.to_type_ident(name);

            let mut comp_tokens = TokenStream::new();
            for c in components {
                let comp_field_ident = generator.to_value_ident(&c.component.id);
                let comp_ty_ident =
                    Asn1ResolvedType::generate_name_maybe_aux_type(&c.component.ty, generator)?;
                let comp_token = if c.optional {
                    quote! {
                        pub #comp_field_ident: Option<#comp_ty_ident>,
                    }
                } else {
                    quote! {
                        pub #comp_field_ident: #comp_ty_ident,
                    }
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
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        if let ResolvedConstructedType::Choice { ref components, .. } = self {
            let type_name = generator.to_type_ident(name);
            let mut comp_tokens = TokenStream::new();
            for c in components {
                let comp_variant_ident = generator.to_type_ident(&c.id);
                let comp_variant_ty_ident =
                    Asn1ResolvedType::generate_name_maybe_aux_type(&c.ty, generator)?;
                let comp_token = quote! {
                    #comp_variant_ident(#comp_variant_ty_ident),
                };
                comp_tokens.extend(comp_token);
            }
            Ok(quote! {
                pub enum #type_name {
                    #comp_tokens
                }
            })
        } else {
            Ok(TokenStream::new())
        }
    }
    pub(crate) fn generate_sequence_of(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        if let ResolvedConstructedType::SequenceOf { ref ty, .. } = self {
            let seq_of_type_ident = generator.to_type_ident(name);
            let seq_of_type = Asn1ResolvedType::generate_name_maybe_aux_type(ty, generator)?;

            Ok(quote! {
                struct #seq_of_type_ident(Vec<#seq_of_type>);
            })
        } else {
            Ok(TokenStream::new())
        }
    }
}
