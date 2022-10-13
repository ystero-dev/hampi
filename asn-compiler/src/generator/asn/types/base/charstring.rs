//! Generator code for Base Type Asn1ResolvedCharacterString

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::error::Error;

use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::Asn1ResolvedCharacterString;

impl Asn1ResolvedCharacterString {
    pub(crate) fn generate(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        let struct_name = generator.to_type_ident(name);
        let char_str_type: proc_macro2::TokenStream =
            format!("\"{}\"", self.str_type).parse().unwrap();

        let mut ty_attributes = quote! { type = #char_str_type };

        if self.size.is_some() {
            let sz_attributes = self.size.as_ref().unwrap().get_ty_size_constraints_attrs();
            ty_attributes.extend(sz_attributes);
        }

        let vis = generator.get_visibility_tokens();
        let dir = generator.generate_derive_tokens();

        let struct_tokens = quote! {
            #dir
            #[asn(#ty_attributes)]
            #vis struct #struct_name(#vis String);
        };

        Ok(struct_tokens)
    }

    pub(crate) fn generate_ident_and_aux_type(
        &self,
        generator: &mut Generator,
    ) -> Result<Ident, Error> {
        let unique_name = generator.get_unique_name("CharacterString");

        let item = self.generate(&unique_name, generator)?;
        generator.aux_items.push(item);

        Ok(generator.to_type_ident(&unique_name))
    }
}
