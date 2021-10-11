//! Handling of Code generation for CHOICE ASN.1 Types

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::error::Error;
use crate::generator::Generator;
use crate::resolver::asn::structs::types::{
    constructed::{ResolvedComponent, ResolvedConstructedType},
    Asn1ResolvedType,
};

// Following is a Private structure only used in this module.
struct ChoiceComponentToken {
    variant: Ident,
    ty: Ident,
    key: i128,
}

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
            let mut choice_tokens = TokenStream::new();

            // First Get The Type Names
            let type_name = generator.to_type_ident(name);

            // Get All the required information from the root_components
            let root_tokens =
                ResolvedConstructedType::get_component_tokens(root_components, name, generator)?;

            // Get all the required information from the additions if present
            let addition_tokens = if additions.is_some() {
                let additions = additions.as_ref().unwrap();
                let addition_tokens =
                    ResolvedConstructedType::get_component_tokens(&additions, name, generator)?;
                Some(addition_tokens)
            } else {
                None
            };

            let struct_tokens =
                ResolvedConstructedType::generate_struct_tokens_for_asn_choice_type(
                    &type_name,
                    &root_tokens,
                    &addition_tokens,
                )?;

            let impl_tokens = ResolvedConstructedType::generate_impl_tokens_for_asn_choice_type(
                &type_name,
                &root_tokens,
                &addition_tokens,
            )?;

            choice_tokens.extend(struct_tokens);
            choice_tokens.extend(impl_tokens);

            Ok(choice_tokens)
        } else {
            Err(code_generate_error!("Constructed Type is not a `CHOICE`"))
        }
    }

    fn generate_struct_tokens_for_asn_choice_type(
        type_name: &Ident,
        root_tokens: &Vec<ChoiceComponentToken>,
        addition_tokens: &Option<Vec<ChoiceComponentToken>>,
    ) -> Result<TokenStream, Error> {
        let mut root_comp_tokens = TokenStream::new();
        for token in root_tokens {
            let variant_ident = token.variant.clone();
            let ty_ident = token.ty.clone();
            let key_token: TokenStream = format!("{}", token.key).parse().unwrap();
            let extension_token = quote! { false };
            let field_attributes =
                quote! { #[asn(key = #key_token, extension = #extension_token)] };
            let comp_token = quote! {
                #field_attributes
                #variant_ident(#ty_ident),
            };
            root_comp_tokens.extend(comp_token);
        }

        let mut addition_comp_tokens = quote! {
            // Choice Additions
        };

        if addition_tokens.is_some() {
            let addition_tokens = addition_tokens.as_ref().unwrap();
            for token in addition_tokens {
                let variant_ident = token.variant.clone();
                let ty_ident = token.ty.clone();
                let key_token: TokenStream = format!("{}", token.key).parse().unwrap();
                let extension_token = quote! { true };
                let field_attributes =
                    quote! { #[asn(key = #key_token, extension = #extension_token)] };
                let comp_token = quote! {
                    #field_attributes
                    #variant_ident(#ty_ident),
                };
                addition_comp_tokens.extend(comp_token);
            }
        }

        // Attributes Tokens (Let's not Get rid of them as yet, they will be required if we decide
        // to go the `#derive` Path
        let lb_token = quote! { lb = 0 };
        let ub_token: TokenStream = format!("{}", root_tokens.len() - 1).parse().unwrap();
        let ub_token = quote! { ub = #ub_token };
        let additions: TokenStream = if addition_tokens.is_some() {
            quote! { true }
        } else {
            quote! { false }
        };
        let additions = quote! { additions = #additions };

        let ty_attributes = quote! { #[asn(#lb_token, #ub_token, #additions)] };

        Ok(quote! {
            #[derive(Debug, AperCodec)]
            #ty_attributes
            pub enum #type_name {
                #root_comp_tokens
                #addition_comp_tokens
            }
        })
    }

    fn generate_impl_tokens_for_asn_choice_type(
        type_name: &Ident,
        root_tokens: &Vec<ChoiceComponentToken>,
        addition_tokens: &Option<Vec<ChoiceComponentToken>>,
    ) -> Result<TokenStream, Error> {
        let lb_token = quote! { 0i128 };
        let ub_token: proc_macro2::TokenStream =
            format!("{}i128", root_tokens.len() - 1).parse().unwrap();

        let extension_present = if addition_tokens.is_some() {
            quote! { true }
        } else {
            quote! { false }
        };

        let root_comp_tokens =
            ResolvedConstructedType::choice_component_impl_decode_aper_tokens(&root_tokens)?;

        let additional_comps_tokens = if addition_tokens.is_some() {
            ResolvedConstructedType::choice_component_impl_decode_aper_tokens(
                &addition_tokens.as_ref().unwrap(),
            )?
        } else {
            quote! { _ => Err(AperCodecError) }
        };

        let impl_tokens = quote! {
            impl AperCodec for #type_name {

                fn decode(decoder: &mut codecs::aper::AperDecoder) -> Result<Self, AperCodecError> {
                    let (choice_idx, is_extension) = decoder.decode_choice(#lb_token, #ub_token, #extension_present)?;

                    if is_extension {
                        match choice_idx {
                            #root_comp_tokens
                        }
                    } else {
                        match choice_idx {
                            #additional_comps_tokens
                        }
                    }
                }
            }
        };
        Ok(impl_tokens)
    }

    fn choice_component_impl_decode_aper_tokens(
        components: &Vec<ChoiceComponentToken>,
    ) -> Result<TokenStream, Error> {
        let mut tokens = TokenStream::new();
        if !components.is_empty() {
            for c in components {
                let variant = c.variant.clone();
                let ty = c.ty.clone();
                let key: proc_macro2::TokenStream = format!("{}i128", c.key).parse().unwrap();

                tokens.extend(quote! {
                    #key => Ok(Self::#variant(#ty::decode(decoder))),
                });
            }
            Ok(tokens)
        } else {
            Ok(quote! { _ => Err(AperCodecError) })
        }
    }

    fn get_component_tokens(
        components: &Vec<ResolvedComponent>,
        name: &str,
        generator: &mut Generator,
    ) -> Result<Vec<ChoiceComponentToken>, Error> {
        let mut out_components = vec![];
        for (i, c) in components.iter().enumerate() {
            let comp_variant_ident = generator.to_type_ident(&c.id);
            let input_comp_type_ident = format!("{}{}", name, c.id);
            let comp_variant_ty_ident = Asn1ResolvedType::generate_name_maybe_aux_type(
                &c.ty,
                generator,
                Some(&input_comp_type_ident),
            )?;

            out_components.push(ChoiceComponentToken {
                variant: comp_variant_ident,
                ty: comp_variant_ty_ident,
                key: i as i128,
            });
        }
        Ok(out_components)
    }
}
