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
        let struct_tokens = quote! {
            #[derive(Debug, AperCodec)]
            #[asn(type = #char_str_type)]
            pub struct #struct_name(String);
        };

        Ok(struct_tokens)
    }

    pub(crate) fn generate_ident_and_aux_type(
        &self,
        generator: &mut Generator,
    ) -> Result<Ident, Error> {
        let unique_name = generator.to_unique_name("CharacterString");

        let item = self.generate(&unique_name, generator)?;
        generator.aux_items.push(item);

        Ok(generator.to_type_ident(&unique_name))
    }
}
