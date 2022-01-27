//! Mainly 'generator' code for `Asn1ResolvedEnumerated`

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::error::Error;

use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::Asn1ResolvedEnumerated;

impl Asn1ResolvedEnumerated {
    pub(crate) fn generate(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        let struct_name = generator.to_type_ident(name);
        let inner_type = generator.to_inner_type(self.bits, self.signed);

        let named_values = self.generate_named_values(generator)?;

        let mut ty_attributes = quote! { type = "ENUMERATED" };
        if self.extensible {
            ty_attributes.extend(quote! { , extensible = true });
        }
        let ub = format!("{}", self.named_root_values.len() - 1);
        ty_attributes.extend(quote! { , lb = "0" });
        ty_attributes.extend(quote! { , ub =  #ub  });

        let struct_tokens = quote! {
            #[derive(Debug, AperCodec)]
            #[asn(#ty_attributes)]
            pub struct #struct_name(#inner_type);

            impl #struct_name {
                #named_values
            }
        };

        Ok(struct_tokens)
    }

    fn generate_named_values(&self, generator: &Generator) -> Result<TokenStream, Error> {
        let mut tokens = TokenStream::new();
        for (name, value) in &self.named_root_values {
            let const_name = generator.to_const_ident(&name);
            let value_literal = generator.to_suffixed_literal(self.bits, self.signed, *value);
            let ty = generator.to_inner_type(self.bits, self.signed);
            let const_tokens = quote! {
                const #const_name: #ty =  #value_literal ;
            };
            tokens.extend(const_tokens);
        }

        Ok(tokens)
    }

    pub(crate) fn generate_ident_and_aux_type(
        &self,
        generator: &mut Generator,
    ) -> Result<Ident, Error> {
        let unique_name = generator.get_unique_name("ENUMERATED");

        let item = self.generate(&unique_name, generator)?;
        generator.aux_items.push(item);

        Ok(generator.to_type_ident(&unique_name))
    }
}
