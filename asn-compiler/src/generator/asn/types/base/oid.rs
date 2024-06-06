//! Generator code for 'Asn1ResolvedBoolean'.

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::Asn1ResolvedObjectIdentifier;
use anyhow::Result;

impl Asn1ResolvedObjectIdentifier {
    pub(crate) fn generate(&self, name: &str, generator: &mut Generator) -> Result<TokenStream> {
        let type_name = generator.to_type_ident(name);

        let vis = generator.get_visibility_tokens();
        let dir = generator.generate_derive_tokens();

        Ok(quote! {
            #dir
            #[asn(type = "OBJECT-IDENTIFIER")]
            #vis struct #type_name(Vec<u32>);
        })
    }

    pub(crate) fn generate_ident_and_aux_type(
        &self,
        generator: &mut Generator,
        input: Option<&String>,
    ) -> Result<Ident> {
        let unique_name = if let Some(unique_name) = input {
            unique_name.to_string()
        } else {
            generator.get_unique_name("OBJECT IDENTIFIER")
        };

        let item = self.generate(&unique_name, generator)?;
        generator.aux_items.push(item);

        Ok(generator.to_type_ident(&unique_name))
    }
}
