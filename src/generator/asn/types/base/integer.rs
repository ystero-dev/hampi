//! Mainly 'generator' code for `Asn1ResolvedInteger`

use proc_macro2::TokenStream;
use quote::quote;

use crate::error::Error;

use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::Asn1ResolvedInteger;

impl Asn1ResolvedInteger {
    pub(crate) fn generate(&self, name: &str, generator: &Generator) -> Result<TokenStream, Error> {
        let struct_name = generator.to_type_ident(name);
        let inner_type = generator.to_inner_type(self.bits, self.signed);
        let struct_tokens = quote! {
            #[derive(Debug)]
            pub struct #struct_name(#inner_type);
        };

        Ok(struct_tokens)
    }
}
