//! Generator code for Base Type Asn1ResolvedOctetString

use proc_macro2::TokenStream;
use quote::quote;

use crate::error::Error;

use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::Asn1ResolvedOctetString;

impl Asn1ResolvedOctetString {
    pub(crate) fn generate(&self, name: &str, generator: &Generator) -> Result<TokenStream, Error> {
        let struct_name = generator.to_type_ident(name);
        let struct_tokens = quote! {
            #[derive(Debug)]
            pub struct #struct_name(Vec<u8>);
        };

        Ok(struct_tokens)
    }
}
