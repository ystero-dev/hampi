//! Code Generation module

use heck::{ShoutySnakeCase, SnakeCase};
use proc_macro2::{Ident, Literal, Span, TokenStream};

use quote::quote;

use crate::error::Error;
use crate::resolver::Resolver;

use crate::resolver::asn::structs::types::Asn1ResolvedType;

#[derive(Debug)]
pub(crate) struct Generator {
    // Generated Tokens for the module.
    pub(crate) items: Vec<TokenStream>,

    // Rust module name.
    pub(crate) name: String,

    // A counter to uniquify certain names
    pub(crate) counter: usize,

    // Auxillary Items: These are structs/that are referenced inside constructed type.
    pub(crate) aux_items: Vec<TokenStream>,
}

impl Generator {
    pub(crate) fn new(name: &str) -> Self {
        Generator {
            items: vec![],
            name: name.to_string(),
            counter: 1,
            aux_items: vec![],
        }
    }

    // Generates the code using the information from the `Resolver`. Returns a String
    // containing all the code (which is basically a `format!` of the `TokenStream`.
    pub(crate) fn generate(&mut self, resolver: &Resolver) -> Result<String, Error> {
        // FIXME: Not sure how to make sure the crates defined here are a dependency.
        // May be can just do with documenting it.
        let use_tokens = self.generate_use_tokens();
        self.items.push(use_tokens);

        let mut items = vec![];
        for (k, t) in resolver.get_resolved_types() {
            let item = Asn1ResolvedType::generate_for_type(k, t, self)?;
            if item.is_some() {
                items.push(item.unwrap())
            }
        }

        for aux in &self.aux_items {
            items.push(aux.clone())
        }

        self.items.extend(items);

        Ok(format!(
            "{}",
            self.items
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join("\n\n")
        ))
    }

    pub(crate) fn to_type_ident(&self, name: &str) -> Ident {
        Ident::new(&name.replace("-", "_").replace(" ", "_"), Span::call_site())
    }

    pub(crate) fn to_const_ident(&self, name: &str) -> Ident {
        Ident::new(&name.to_shouty_snake_case(), Span::call_site())
    }

    pub(crate) fn to_value_ident(&self, name: &str) -> Ident {
        let mut val = name.to_snake_case();
        if val == "type".to_string() {
            val = "typ".to_string()
        }
        Ident::new(&val, Span::call_site())
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

    pub(crate) fn to_unique_name(&mut self, name: &str) -> String {
        self.counter += 1;

        format!("{} {}", name, self.counter)
    }

    fn generate_use_tokens(&self) -> TokenStream {
        quote! {
            #![allow(dead_code, unreachable_patterns, non_camel_case_types)]

            use bitvec::vec::BitVec;
            use bitvec::order::Msb0;

            // FIXME: Do this based on the Codec to be supported, right now we are only supporting
            // APER Codec.
            use asn1_codecs_derive::AperCodec;
        }
    }
}
