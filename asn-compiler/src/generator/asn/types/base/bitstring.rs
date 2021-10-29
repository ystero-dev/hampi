//! Generator code for Base Type Asn1ResolvedBitString

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::error::Error;

use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::Asn1ResolvedBitString;

impl Asn1ResolvedBitString {
    pub(crate) fn generate(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        let sz_extensible = if self.size.is_some() {
            let sz = self.size.as_ref().unwrap();
            let ext = sz.has_extension();
            quote! { #ext }
        } else {
            quote! {false}
        };

        let (lb, ub) = if self.size.is_some() {
            let sz = self.size.as_ref().unwrap();
            (Some(sz.root_values.min()), Some(sz.root_values.max()))
        } else {
            (None, None)
        };
        let mut ty_attributes = quote! { type = "BITSTRING" };
        ty_attributes.extend(quote! { , sz_extensible = #sz_extensible });

        if lb.is_some() {
            let lb = lb.unwrap();
            ty_attributes.extend(quote! { , lb = #lb });
        }
        if ub.is_some() {
            let ub = ub.unwrap();
            ty_attributes.extend(quote! { , ub = #ub });
        }

        let struct_name = generator.to_type_ident(name);
        let struct_tokens = quote! {
            #[derive(Debug, AperCodec)]
            #[asn(#ty_attributes)]
            pub struct #struct_name(BitVec<Msb0, usize>);
        };

        Ok(struct_tokens)
    }

    pub(crate) fn generate_ident_and_aux_type(
        &self,
        generator: &mut Generator,
    ) -> Result<Ident, Error> {
        let unique_name = generator.to_unique_name("BIT STRING");

        let item = self.generate(&unique_name, generator)?;
        generator.aux_items.push(item);

        Ok(generator.to_type_ident(&unique_name))
    }
}
