//! Generator code for Base Type Asn1ResolvedBitString

use proc_macro2::TokenStream;
use quote::quote;

use crate::error::Error;

use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::Asn1ResolvedBitString;

impl Asn1ResolvedBitString {
    pub(crate) fn generate(&self, name: &str, generator: &Generator) -> Result<TokenStream, Error> {
        let struct_name = generator.to_type_ident(name);
        let struct_tokens = quote! {
            #[derive(Debug)]
            pub struct #struct_name(BitVec<Msb0, usize>);
        };

        Ok(struct_tokens)
    }
}
