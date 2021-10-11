//! Implementation of Code Generation for ASN.1 `SEQUENCE` Type.

use proc_macro2::TokenStream;
use quote::quote;

use crate::error::Error;
use crate::generator::Generator;
use crate::resolver::asn::structs::types::{
    constructed::ResolvedConstructedType, Asn1ResolvedType,
};

impl ResolvedConstructedType {
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
                let input_comp_ty_ident = format!("{}{}", name, c.component.id);
                let comp_ty_ident = Asn1ResolvedType::generate_name_maybe_aux_type(
                    &c.component.ty,
                    generator,
                    Some(&input_comp_ty_ident),
                )?;
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
                #[derive(Debug, AperCodec)]
                #[asn(type = "SEQUENCE")]
                pub struct #type_name {
                    #comp_tokens
                }
            })
        } else {
            Ok(TokenStream::new())
        }
    }
}
