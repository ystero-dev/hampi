//! Code Generation module

use heck::CamelCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[derive(Debug)]
pub(crate) struct Generator {
    // Generated Tokens for the module.
    tokens: TokenStream,

    // Rust module name.
    name: String,
}

impl Generator {
    pub(crate) fn new(name: &str) -> Self {
        Generator {
            tokens: TokenStream::new(),
            name: name.to_string(),
        }
    }

    pub(crate) fn to_type_name(&self, name: &str) -> Ident {
        Ident::new(&name.to_camel_case(), Span::call_site())
    }

    pub(crate) fn to_inner_type(&self, bits: u8, signed: bool) -> TokenStream {
        if signed {
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
