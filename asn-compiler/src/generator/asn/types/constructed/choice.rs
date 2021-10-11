//! Handling of Code generation for CHOICE ASN.1 Types

use proc_macro2::TokenStream;
use quote::quote;

use crate::error::Error;
use crate::generator::Generator;
use crate::resolver::asn::structs::types::{
    constructed::ResolvedConstructedType, Asn1ResolvedType,
};

impl ResolvedConstructedType {
    pub(crate) fn generate_choice(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        if let ResolvedConstructedType::Choice {
            ref root_components,
            ref additions,
            ..
        } = self
        {
            let type_name = generator.to_type_ident(name);
            let mut root_comp_tokens = TokenStream::new();
            for (i, c) in root_components.iter().enumerate() {
                let choice_idx: proc_macro2::TokenStream = format!("\"{}\"", i).parse().unwrap();
                let fld_attrs = quote!( #[asn(choice_idx = #choice_idx)] );
                let comp_variant_ident = generator.to_type_ident(&c.id);
                let input_comp_ty_ident = format!("{}{}", name, c.id);
                let comp_variant_ty_ident = Asn1ResolvedType::generate_name_maybe_aux_type(
                    &c.ty,
                    generator,
                    Some(&input_comp_ty_ident),
                )?;
                let comp_token = quote! {
                    #fld_attrs
                    #comp_variant_ident(#comp_variant_ty_ident),
                };
                root_comp_tokens.extend(comp_token);
            }

            let mut addition_comp_tokens = TokenStream::new();
            if additions.is_some() {
                for (i, a) in additions.as_ref().unwrap().iter().enumerate() {
                    let choice_idx: proc_macro2::TokenStream =
                        format!("\"{}\"", i).parse().unwrap();
                    let fld_attrs = quote!( #[asn(choice_idx = #choice_idx, extension)] );
                    let input_comp_ty_ident = format!("{}{}", name, a.id);
                    let comp_variant_ident = generator.to_type_ident(&a.id);
                    let comp_variant_ty_ident = Asn1ResolvedType::generate_name_maybe_aux_type(
                        &a.ty,
                        generator,
                        Some(&input_comp_ty_ident),
                    )?;
                    let comp_token = quote! {
                        #fld_attrs
                        #comp_variant_ident(#comp_variant_ty_ident),
                    };
                    addition_comp_tokens.extend(comp_token);
                }
            }
            let ty_attributes = quote! { type = "CHOICE" };

            let root_comps_len: proc_macro2::TokenStream =
                format!("\"{}\"", root_components.len() - 1)
                    .parse()
                    .unwrap();
            let lower_bound = quote! { , lb = "0" };
            let upper_bound = quote! { , ub = #root_comps_len };

            let value_extensible = if additions.is_some() {
                quote! {
                    , value_extensibe
                }
            } else {
                quote! {}
            };
            let ty_attributes =
                quote! { #ty_attributes #lower_bound #upper_bound #value_extensible };

            Ok(quote! {
                #[derive(Debug, AperCodec)]
                #[asn(#ty_attributes)]
                pub enum #type_name {
                    #root_comp_tokens
                    #addition_comp_tokens
                }
            })
        } else {
            Ok(TokenStream::new())
        }
    }
}
