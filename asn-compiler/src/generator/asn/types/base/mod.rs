//! Handling of Code Generation for 'Built-in' Types

mod integer;

mod enumerated;

mod bitstring;

mod boolean;

mod octetstring;

mod charstring;

mod null;

mod oid;

mod real;

use proc_macro2::{Ident, TokenStream};

use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::ResolvedBaseType;
use anyhow::Result;

impl ResolvedBaseType {
    pub(crate) fn generate_for_base_type(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream> {
        match self {
            ResolvedBaseType::Integer(ref i) => i.generate(name, generator),
            ResolvedBaseType::Enum(ref e) => e.generate(name, generator),
            ResolvedBaseType::BitString(ref b) => b.generate(name, generator),
            ResolvedBaseType::Boolean(ref b) => b.generate(name, generator),
            ResolvedBaseType::OctetString(ref o) => o.generate(name, generator),
            ResolvedBaseType::CharacterString(ref c) => c.generate(name, generator),
            ResolvedBaseType::Null(ref n) => n.generate(name, generator),
            ResolvedBaseType::Real(ref r) => r.generate(name, generator),
            ResolvedBaseType::ObjectIdentifier(ref o) => o.generate(name, generator),
        }
    }

    pub(crate) fn generate_ident_and_aux_type_for_base(
        &self,
        generator: &mut Generator,
        input: Option<&String>,
    ) -> Result<Ident> {
        match self {
            ResolvedBaseType::Integer(ref i) => i.generate_ident_and_aux_type(generator, input),
            ResolvedBaseType::Enum(ref e) => e.generate_ident_and_aux_type(generator, input),
            ResolvedBaseType::BitString(ref b) => b.generate_ident_and_aux_type(generator, input),
            ResolvedBaseType::Boolean(ref b) => b.generate_ident_and_aux_type(generator, input),
            ResolvedBaseType::OctetString(ref o) => o.generate_ident_and_aux_type(generator, input),
            ResolvedBaseType::CharacterString(ref c) => {
                c.generate_ident_and_aux_type(generator, input)
            }
            ResolvedBaseType::Null(ref n) => n.generate_ident_and_aux_type(generator, input),
            ResolvedBaseType::Real(ref r) => r.generate_ident_and_aux_type(generator, input),
            ResolvedBaseType::ObjectIdentifier(ref o) => {
                o.generate_ident_and_aux_type(generator, input)
            }
        }
    }
}
