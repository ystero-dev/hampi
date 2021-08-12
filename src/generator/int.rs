//! Code Generation module

use heck::CamelCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::error::Error;
use crate::resolver::Resolver;

use crate::resolver::asn::structs::types::{base::ResolvedBaseType, Asn1ResolvedType};

#[derive(Debug)]
pub(crate) struct Generator {
    // Generated Tokens for the module.
    tokens: TokenStream,

    // Rust module name.
    name: String,

    // A counter to uniquify certain names
    counter: usize,
}

impl Generator {
    pub(crate) fn new(name: &str) -> Self {
        Generator {
            tokens: TokenStream::new(),
            name: name.to_string(),
            counter: 1,
        }
    }

    // Generates the code using the information from the `Resolver`. Returns a String
    // containing all the code (which is basically a `format!` of the `TokenStream`.
    pub(crate) fn generate(&mut self, resolver: &Resolver) -> Result<String, Error> {
        for (k, t) in resolver.get_resolved_types() {
            match t {
                // TODO: When we support 'ALL' base types, move this function to
                // `generator/asn/types/base/mod.rs`
                Asn1ResolvedType::Base(ResolvedBaseType::Integer(ref i)) => {
                    self.tokens.extend(i.generate(k, &self)?);
                }
                _ => {}
            }
        }
        Ok(format!("{}", self.tokens))
    }

    pub(crate) fn to_type_name(&self, name: &str) -> Ident {
        Ident::new(&name.to_camel_case(), Span::call_site())
    }

    pub(crate) fn to_inner_type(&self, bits: u8, signed: bool) -> TokenStream {
        if !signed {
            match bits {
                8 => quote!(u8),
                16 => quote!(u16),
                32 => quote!(u32),
                64 => quote!(u64),
                _ => quote!(u64),
            }
        } else {
            match bits {
                8 => quote!(i8),
                16 => quote!(i16),
                32 => quote!(i32),
                64 => quote!(i64),
                _ => quote!(i64),
            }
        }
    }
}
