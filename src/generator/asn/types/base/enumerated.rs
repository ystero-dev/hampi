//! Mainly 'generator' code for `Asn1ResolvedEnumerated`

use proc_macro2::TokenStream;
use quote::quote;

use crate::error::Error;

use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::Asn1ResolvedEnumerated;

impl Asn1ResolvedEnumerated {
    pub(crate) fn generate(&self, name: &str, generator: &Generator) -> Result<TokenStream, Error> {
        let struct_name = generator.to_type_ident(name);
        let inner_type = generator.to_inner_type(self.bits, self.signed);

        let named_values = self.generate_named_values(generator)?;
        let struct_tokens = quote! {
            #[derive(Debug)]
            pub struct #struct_name(#inner_type);

            impl #struct_name {
                #named_values
            }
        };

        Ok(struct_tokens)
    }

    fn generate_named_values(&self, generator: &Generator) -> Result<TokenStream, Error> {
        let mut tokens = TokenStream::new();
        for (name, &value) in &self.named_values {
            let const_name = generator.to_const_ident(&name);
            let value_literal = generator.to_suffixed_literal(self.bits, self.signed, value);
            let ty = generator.to_inner_type(self.bits, self.signed);
            let const_tokens = quote! {
                const #const_name: #ty =  #value_literal ;
            };
            tokens.extend(const_tokens);
        }

        Ok(tokens)
    }
}
