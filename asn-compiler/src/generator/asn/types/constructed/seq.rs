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
        if let ResolvedConstructedType::Sequence {
            ref components,
            ref extensible,
            ref additions,
            ..
        } = self
        {
            let type_name = generator.to_type_ident(name);

            let extensible = if *extensible {
                quote! { true }
            } else {
                quote! { false }
            };

            let vis = generator.get_visibility_tokens();

            let mut comp_tokens = TokenStream::new();
            let mut optional_fields = 0;
            for c in components {
                let comp_field_ident = generator.to_value_ident(&c.component.id);
                let comp_ty_suffix = generator.to_type_ident(&c.component.id);
                let input_comp_ty_ident = format!("{}{}", name, comp_ty_suffix);
                let comp_ty_ident = Asn1ResolvedType::generate_name_maybe_aux_type(
                    &c.component.ty,
                    generator,
                    Some(&input_comp_ty_ident),
                )?;
                let mut fld_attrs = vec![];

                let fld_tokens = if c.optional {
                    let idx: proc_macro2::TokenStream =
                        format!("{}", optional_fields).parse().unwrap();
                    fld_attrs.push(quote! { optional_idx = #idx,  });

                    optional_fields += 1;

                    quote! { #vis #comp_field_ident: Option<#comp_ty_ident>, }
                } else {
                    quote! { #vis #comp_field_ident: #comp_ty_ident, }
                };

                if c.key_field {
                    fld_attrs.push(quote! { key_field = true })
                }

                let fld_attr_tokens = if !fld_attrs.is_empty() {
                    quote! { #[asn(#(#fld_attrs),*)] }
                } else {
                    quote! {}
                };

                comp_tokens.extend(quote! {
                    #fld_attr_tokens #fld_tokens
                });
            }

            if additions.is_some() {
                log::warn!("Fields for some sequence additions may not be generated!");
            };

            let mut ty_tokens = quote! { type = "SEQUENCE", extensible = #extensible };

            if optional_fields > 0 {
                let optflds: proc_macro2::TokenStream =
                    format!("{}", optional_fields).parse().unwrap();
                ty_tokens.extend(quote! { , optional_fields = #optflds });
            }

            let dir = generator.generate_derive_tokens();
            Ok(quote! {
                #dir
                #[asn(#ty_tokens)]
                #vis struct #type_name {
                    #comp_tokens
                }
            })
        } else {
            Ok(TokenStream::new())
        }
    }
}
