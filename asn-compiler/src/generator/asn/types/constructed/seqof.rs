//! Implementation of Code Generation for `SEQUENCE OF` ASN Type

use proc_macro2::TokenStream;
use quote::quote;

use crate::generator::Generator;
use crate::resolver::asn::structs::types::{
    constructed::ResolvedConstructedType, Asn1ResolvedType,
};
use anyhow::Result;

impl ResolvedConstructedType {
    pub(crate) fn generate_sequence_of(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream> {
        if let ResolvedConstructedType::SequenceOf {
            ref ty,
            ref size_values,
            ..
        } = self
        {
            let seq_of_type_ident = generator.to_type_ident(name);
            let input_type_name = format!("{}_Entry", name);

            let mut ty_attrs = quote! { type = "SEQUENCE-OF" };
            if size_values.is_some() {
                ty_attrs.extend(
                    size_values
                        .as_ref()
                        .unwrap()
                        .get_ty_size_constraints_attrs(),
                )
            }

            let seq_of_type = Asn1ResolvedType::generate_name_maybe_aux_type(
                ty,
                generator,
                Some(&input_type_name),
            )?;

            let vis = generator.get_visibility_tokens();
            let dir = generator.generate_derive_tokens();

            Ok(quote! {
                #dir
                #[asn(#ty_attrs)]
                #vis struct #seq_of_type_ident(#vis Vec<#seq_of_type>);
            })
        } else {
            Ok(TokenStream::new())
        }
    }
}
