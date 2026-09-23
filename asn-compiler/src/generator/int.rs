//! Code Generation module

use std::collections::{HashMap, HashSet};

use anyhow::Result;
use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;

use lazy_static::lazy_static;

use crate::resolver::Resolver;

use crate::resolver::asn::structs::{
    types::{
        base::ResolvedBaseType, constructed::ResolvedConstructedType, Asn1ResolvedType,
        ResolvedSetType,
    },
    values::Asn1ResolvedValue,
};

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

    // Names of the resolved types that contain a `REAL` (directly or through a
    // reference). `f64` is not `Eq`, so `Eq` must not be derived for these types.
    pub(crate) real_types: HashSet<String>,
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
            real_types: HashSet::new(),
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

        // Find out the types that have a `REAL` in them before generating any code.
        let resolved_types = resolver.get_resolved_types();
        self.real_types = Self::find_types_containing_real(&resolved_types);

        // Now get the types
        for (k, t) in resolved_types {
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
        self.generate_derive_tokens_skip_eq("", false)
    }

    // Same as `generate_derive_tokens`, but leaves out `Eq` when `skip_eq` is true. Used for
    // types that contain a `REAL` (`f64`), which can only be `PartialEq`. `name` is the name of
    // the type being generated, used for the warning when `Eq` is asked for but is skipped.
    pub(crate) fn generate_derive_tokens_skip_eq(&self, name: &str, skip_eq: bool) -> TokenStream {
        let mut tokens = vec![];
        for codec in &self.codecs {
            let codec_token = CODEC_TOKENS.get(codec).unwrap();
            tokens.push(codec_token.to_string());
        }

        for derive in &self.derives {
            if derive == &Derive::All {
                for (d, derive_token) in DERIVE_TOKENS.iter() {
                    if skip_eq && d == &Derive::Eq {
                        log::warn!(
                            "Not deriving `Eq` for type `{}` as it contains a `REAL`.",
                            name
                        );
                        continue;
                    }
                    tokens.push(derive_token.to_string());
                }
            } else if skip_eq && derive == &Derive::Eq {
                log::warn!(
                    "Not deriving `Eq` for type `{}` as it contains a `REAL`.",
                    name
                );
                continue;
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

    // A type can refer to another type that contains a `REAL`, which in turn can be referred
    // by some other type and so on. So we keep going over all the types till no new type
    // gets added to the set.
    fn find_types_containing_real(types: &[(&String, &Asn1ResolvedType)]) -> HashSet<String> {
        let mut found = HashSet::new();
        loop {
            let mut changed = false;
            for (name, ty) in types {
                if !found.contains(*name) && Self::type_has_real(ty, &found) {
                    found.insert((*name).clone());
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        found
    }

    fn type_has_real(ty: &Asn1ResolvedType, real_types: &HashSet<String>) -> bool {
        match ty {
            Asn1ResolvedType::Base(ResolvedBaseType::Real(..)) => true,
            Asn1ResolvedType::Base(..) => false,
            Asn1ResolvedType::Reference(ref r) => real_types.contains(r),
            Asn1ResolvedType::Constructed(ref c) => Self::constructed_has_real(c, real_types),
            Asn1ResolvedType::Set(ref s) => Self::set_has_real(s, real_types),
        }
    }

    fn constructed_has_real(c: &ResolvedConstructedType, real_types: &HashSet<String>) -> bool {
        match c {
            ResolvedConstructedType::Choice {
                root_components,
                additions,
                ..
            } => root_components
                .iter()
                .chain(additions.iter().flatten())
                .any(|comp| Self::type_has_real(&comp.ty, real_types)),
            ResolvedConstructedType::Sequence {
                components,
                additions,
                ..
            } => components
                .iter()
                .chain(additions.iter().flatten())
                .any(|comp| Self::type_has_real(&comp.component.ty, real_types)),
            ResolvedConstructedType::SequenceOf { ty, .. } => Self::type_has_real(ty, real_types),
        }
    }

    fn set_has_real(s: &ResolvedSetType, real_types: &HashSet<String>) -> bool {
        s.types
            .values()
            .any(|(_, ty)| Self::type_has_real(ty, real_types))
    }

    pub(crate) fn constructed_type_has_real(&self, c: &ResolvedConstructedType) -> bool {
        Self::constructed_has_real(c, &self.real_types)
    }

    pub(crate) fn set_type_has_real(&self, s: &ResolvedSetType) -> bool {
        Self::set_has_real(s, &self.real_types)
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
