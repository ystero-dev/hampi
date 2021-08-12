//! Generator code for 'Asn1ResolvedBoolean'.

use proc_macro2::TokenStream;
use quote::quote;

use crate::error::Error;
use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::Asn1ResolvedBoolean;

impl Asn1ResolvedBoolean {
    pub(crate) fn generate(&self, name: &str, generator: &Generator) -> Result<TokenStream, Error> {
        let type_name = generator.to_type_ident(name);
        Ok(quote! {
            pub struct #type_name(bool);
        })
    }
}
