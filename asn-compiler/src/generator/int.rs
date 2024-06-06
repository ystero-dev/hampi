//! Code Generation module

use std::collections::HashMap;

use anyhow::Result;
use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;

use lazy_static::lazy_static;

use crate::resolver::Resolver;

use crate::resolver::asn::structs::{types::Asn1ResolvedType, values::Asn1ResolvedValue};

/// Supported Codecs
#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Codec {
    /// Generate code for ASN.1 APER Codec
    Aper,

    /// Generate code for ASN.1 UPER Codec
    Uper,
}

/// Supported Derive Macros
#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Derive {
    /// Generate `Debug` code for the generated strucutres. Generated for all structures by
    /// default.
    Debug,

    /// Generate 'Clone' code for the generated structures.
    Clone,

    /// Generate 'serde::Serialize' code for the generated structures.
    Serialize,

    /// Generate 'serde::Deserialize' code for the generated structures.
    Deserialize,

    /// Generate `Eq` code for the generated structures.
    Eq,

    /// Generate `PartialEq` code for the generated structures.
    PartialEq,

    /// Generate code for all supported derives for the generated structures.
    All,
}

/// Visibility to be used for the generated Structs, Enums etc.
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Visibility {
    /// Visibility is Public
    Public,
    /// Visibility is Crate
    Crate,
    /// Visibility is Private
    Private,
}

lazy_static! {
    static ref CODEC_TOKENS: HashMap<Codec, String> = {
        let mut m = HashMap::new();
        m.insert(Codec::Aper, "asn1_codecs_derive::AperCodec".to_string());
        m.insert(Codec::Uper, "asn1_codecs_derive::UperCodec".to_string());
        m
    };
    static ref DERIVE_TOKENS: HashMap<Derive, String> = {
        let mut m = HashMap::new();
        m.insert(Derive::Debug, "Debug".to_string());
        m.insert(Derive::Clone, "Clone".to_string());
        m.insert(Derive::Serialize, "serde::Serialize".to_string());
        m.insert(Derive::Deserialize, "serde::Deserialize".to_string());
        m.insert(Derive::Eq, "Eq".to_string());
        m.insert(Derive::PartialEq, "PartialEq".to_string());
        m
    };
}

#[derive(Debug)]
pub(crate) struct Generator {
    // Generated Tokens for the module.
    pub(crate) items: Vec<TokenStream>,

    // A counter to uniquify certain names
    pub(crate) counter: usize,

    // Auxillary Items: These are structs/that are referenced inside constructed type.
    pub(crate) aux_items: Vec<TokenStream>,

    // Visibility: Visibility of Generated Items
    pub(crate) visibility: Visibility,

    // codecs
    pub(crate) codecs: Vec<Codec>,

    // Derives
    pub(crate) derives: Vec<Derive>,
}

impl Generator {
    pub(crate) fn new(visibility: &Visibility, codecs: Vec<Codec>, derives: Vec<Derive>) -> Self {
        Generator {
            items: vec![],
            counter: 1,
            aux_items: vec![],
            visibility: visibility.clone(),
            codecs,
            derives,
        }
    }

    // Generates the code using the information from the `Resolver`. Returns a String
    // containing all the code (which is basically a `format!` of the `TokenStream`.
    pub(crate) fn generate(&mut self, resolver: &Resolver) -> Result<String> {
        // FIXME: Not sure how to make sure the crates defined here are a dependency.
        // May be can just do with documenting it.

        // First Get the 'consts' for builtin values.
        let mut items = vec![];
        for (k, v) in resolver.get_resolved_values() {
            let item = Asn1ResolvedValue::generate_const_for_base_value(k, v, self)?;
            if let Some(it) = item {
                items.push(it)
            }
        }

        // Now get the types
        for (k, t) in resolver.get_resolved_types() {
            let item = Asn1ResolvedType::generate_for_type(k, t, self)?;
            if let Some(it) = item {
                items.push(it)
            }
        }

        for aux in &self.aux_items {
            items.push(aux.clone())
        }

        self.items.extend(items);

        Ok(self
            .items
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join("\n\n"))
    }

    pub(crate) fn to_type_ident(&self, name: &str) -> Ident {
        Ident::new(
            &capitalize_first(name).replace(['-', ' '], "_"),
            Span::call_site(),
        )
    }

    pub(crate) fn to_const_ident(&self, name: &str) -> Ident {
        Ident::new(&name.to_shouty_snake_case(), Span::call_site())
    }

    pub(crate) fn to_value_ident(&self, name: &str) -> Ident {
        let mut val = capitalize_first(name).to_snake_case();
        if val == *"type" {
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

    pub(crate) fn get_unique_name(&mut self, name: &str) -> String {
        self.counter += 1;

        format!("{} {}", name, self.counter)
    }

    pub(crate) fn get_visibility_tokens(&self) -> TokenStream {
        match self.visibility {
            Visibility::Public => quote! { pub },
            Visibility::Crate => quote! { pub(crate) },
            Visibility::Private => quote! {},
        }
    }

    pub(crate) fn generate_derive_tokens(&self) -> TokenStream {
        let mut tokens = vec![];
        for codec in &self.codecs {
            let codec_token = CODEC_TOKENS.get(codec).unwrap();
            tokens.push(codec_token.to_string());
        }

        for derive in &self.derives {
            if derive == &Derive::All {
                for derive_token in DERIVE_TOKENS.values() {
                    tokens.push(derive_token.to_string());
                }
            } else {
                let derive_token = DERIVE_TOKENS.get(derive).unwrap();
                tokens.push(derive_token.to_string());
            }
        }

        let token_string = tokens.join(",");

        let derive_token_string = format!("#[derive({})]\n", token_string);
        let derive_token_stream: TokenStream = derive_token_string.parse().unwrap();
        derive_token_stream
    }
}

fn capitalize_first(input: &str) -> String {
    if !input.is_empty() {
        let mut input = input.to_string();
        let (first, _) = input.split_at_mut(1);
        first.make_ascii_uppercase();

        input
    } else {
        input.to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_capitalize_first_empty() {
        let empty = "".to_string();
        let capitalized = capitalize_first(&empty);
        assert_eq!(capitalized, empty);
    }

    #[test]
    fn test_capitalize_first_single_letter() {
        let empty = "a".to_string();
        let capitalized = capitalize_first(&empty);
        assert_eq!(capitalized, "A");
    }

    #[test]
    fn test_capitalize_first_word() {
        let empty = "amfTnlAssociationToAddItem".to_string();
        let capitalized = capitalize_first(&empty);
        assert_eq!(capitalized, "AmfTnlAssociationToAddItem");
    }
}
