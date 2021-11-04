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
        Ok(quote! {
            #[derive(Debug, AperCodec)]
            #[asn(type = "OBJECT-IDENTIFIER")]
            pub struct #type_name;
        })
    }

    pub(crate) fn generate_ident_and_aux_type(
        &self,
        generator: &mut Generator,
    ) -> Result<Ident, Error> {
        let unique_name = generator.to_unique_name("OBJECT IDENTIFIER");

        let item = self.generate(&unique_name, generator)?;
        generator.aux_items.push(item);

        Ok(generator.to_type_ident(&unique_name))
    }
}
