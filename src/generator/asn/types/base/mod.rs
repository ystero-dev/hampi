mod integer;

mod enumerated;

use proc_macro2::TokenStream;

use crate::error::Error;
use crate::generator::Generator;
use crate::resolver::asn::structs::types::{base::ResolvedBaseType, Asn1ResolvedType};

impl Asn1ResolvedType {
    pub(crate) fn generate_for_base_type(
        name: &str,
        base: &ResolvedBaseType,
        generator: &Generator,
    ) -> Result<TokenStream, Error> {
        match base {
            ResolvedBaseType::Integer(ref i) => i.generate(name, generator),
            ResolvedBaseType::Enum(ref e) => e.generate(name, generator),
            _ => Ok(TokenStream::new()),
        }
    }
}
