//! Handling of Code Generation for 'Built-in' Types

mod integer;

mod enumerated;

mod bitstring;

mod boolean;

mod octetstring;

mod charstring;

// TODO: NULL, OBJECT IDENTIFIER

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::error::Error;
use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::ResolvedBaseType;

impl ResolvedBaseType {
    pub(crate) fn generate_for_base_type(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        match self {
            ResolvedBaseType::Integer(ref i) => i.generate(name, generator),
            ResolvedBaseType::Enum(ref e) => e.generate(name, generator),
            ResolvedBaseType::BitString(ref b) => b.generate(name, generator),
            ResolvedBaseType::Boolean(ref b) => b.generate(name, generator),
            ResolvedBaseType::OctetString(ref o) => o.generate(name, generator),
            ResolvedBaseType::CharacterString(ref c) => c.generate(name, generator),
            ResolvedBaseType::Null => Ok(quote! { () }),
            _ => Ok(TokenStream::new()),
        }
    }

    pub(crate) fn generate_ident_and_aux_type_for_base(
        &self,
        generator: &mut Generator,
    ) -> Result<Ident, Error> {
        match self {
            ResolvedBaseType::Integer(ref i) => i.generate_ident_and_aux_type(generator),
            ResolvedBaseType::Enum(ref e) => e.generate_ident_and_aux_type(generator),
            ResolvedBaseType::BitString(ref b) => b.generate_ident_and_aux_type(generator),
            ResolvedBaseType::Boolean(ref b) => b.generate_ident_and_aux_type(generator),
            ResolvedBaseType::OctetString(ref o) => o.generate_ident_and_aux_type(generator),
            ResolvedBaseType::CharacterString(ref c) => c.generate_ident_and_aux_type(generator),
            ResolvedBaseType::Null => {
                let uniq = generator.to_unique_name("NULL");
                Ok(generator.to_type_ident(&uniq))
            }
            _ => {
                // FIXME: TODO Type
                let uniq = generator.to_unique_name("OBJECT IDENTIFIER");
                Ok(generator.to_type_ident(&uniq))
            }
        }
    }
}
