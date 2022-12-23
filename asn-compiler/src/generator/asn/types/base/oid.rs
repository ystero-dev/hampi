//! Generator code for 'Asn1ResolvedBoolean'.

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::error::Error;
use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::Asn1ResolvedObjectIdentifier;

impl Asn1ResolvedObjectIdentifier {
    pub(crate) fn generate(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        let type_name = generator.to_type_ident(name);

        let vis = generator.get_visibility_tokens();
        let dir = generator.generate_derive_tokens();

        Ok(quote! {
            #dir
            #[asn(type = "OBJECT-IDENTIFIER")]
            #vis struct #type_name;
        })
    }

    pub(crate) fn generate_ident_and_aux_type(
        &self,
        generator: &mut Generator,
        input: Option<&String>,
    ) -> Result<Ident, Error> {
        let unique_name = if input.is_none() {
            generator.get_unique_name("OBJECT IDENTIFIER")
        } else {
            input.unwrap().to_string()
        };

        let item = self.generate(&unique_name, generator)?;
        generator.aux_items.push(item);

        Ok(generator.to_type_ident(&unique_name))
    }
}
