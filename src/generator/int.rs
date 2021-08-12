//! Code Generation module

use heck::{CamelCase, ShoutySnakeCase};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;

use crate::error::Error;
use crate::resolver::Resolver;

use crate::resolver::asn::structs::types::Asn1ResolvedType;

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
                Asn1ResolvedType::Base(ref b) => self
                    .tokens
                    .extend(Asn1ResolvedType::generate_for_base_type(k, b, self)?),
                _ => {}
            }
        }
        Ok(format!("{}", self.tokens))
    }

    pub(crate) fn to_type_ident(&self, name: &str) -> Ident {
        Ident::new(&name.to_camel_case(), Span::call_site())
    }

    pub(crate) fn to_const_ident(&self, name: &str) -> Ident {
        Ident::new(&name.to_shouty_snake_case(), Span::call_site())
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

    pub(crate) fn to_suffixed_literal(&self, bits: u8, signed: bool, value: i128) -> Literal {
        if !signed {
            match bits {
                8 => Literal::u8_suffixed(value as u8),
                16 => Literal::u16_suffixed(value as u16),
                32 => Literal::u32_suffixed(value as u32),
                64 => Literal::u64_suffixed(value as u64),
                _ => Literal::u64_suffixed(value as u64),
            }
        } else {
            match bits {
                8 => Literal::i8_suffixed(value as i8),
                16 => Literal::i16_suffixed(value as i16),
                32 => Literal::i32_suffixed(value as i32),
                64 => Literal::i64_suffixed(value as i64),
                _ => Literal::i64_suffixed(value as i64),
            }
        }
    }
}
