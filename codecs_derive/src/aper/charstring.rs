//! `APER` Code generation for ASN.1 CharacterString Types

use proc_macro2::Span;
use quote::quote;

use crate::attrs::TyCodecParams;

pub(super) fn generate_aper_codec_for_asn_charstring(
    ast: &syn::DeriveInput,
    params: &TyCodecParams,
) -> proc_macro::TokenStream {
    let ty_attr = params.ty.as_ref().unwrap();

    let (decode_fn_name, encode_fn_name): (syn::Ident, syn::Ident) = match ty_attr.value().as_str()
    {
        "UTF8String" => (
            syn::Ident::new("decode_utf8_string", Span::call_site()),
            syn::Ident::new("encode_utf8_string", Span::call_site()),
        ),

        "PrintableString" => (
            syn::Ident::new("decode_printable_string", Span::call_site()),
            syn::Ident::new("encode_printable_string", Span::call_site()),
        ),

        "VisibleString" => (
            syn::Ident::new("decode_visible_string", Span::call_site()),
            syn::Ident::new("encode_visible_string", Span::call_site()),
        ),

        _ => (
            syn::Ident::new("unsupported", Span::call_site()),
            syn::Ident::new("unsupported", Span::call_site()),
        ),
    };

    if decode_fn_name == "unspported" {
        return syn::Error::new_spanned(ty_attr, "Character String Type is not supported.")
            .to_compile_error()
            .into();
    }

    let name = &ast.ident;

    let ty = if let syn::Data::Struct(ref d) = &ast.data {
        match d.fields {
            syn::Fields::Unnamed(ref f) => {
                if f.unnamed.len() == 1 {
                    let first = f.unnamed.first().unwrap();
                    Some(first.ty.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    };

    if ty.is_none() {
        return syn::Error::new_spanned(ast, format!("{} Should be a Unit Struct.", name))
            .to_compile_error()
            .into();
    }

    let sz_lb = if params.sz_lb.is_some() {
        let lb = params
            .sz_lb
            .as_ref()
            .unwrap()
            .value()
            .parse::<i128>()
            .unwrap();
        quote! {
            Some(#lb)
        }
    } else {
        quote! {
            None
        }
    };
    let sz_ub = if params.sz_ub.is_some() {
        let ub = params
            .sz_ub
            .as_ref()
            .unwrap()
            .value()
            .parse::<i128>()
            .unwrap();
        quote! {
            Some(#ub)
        }
    } else {
        quote! {
            None
        }
    };
    let sz_ext = if params.sz_ext.is_some() {
        let ext = params.sz_ext.as_ref();
        quote! {
            #ext
        }
    } else {
        quote! {
            false
        }
    };

    let tokens = quote! {

        impl asn1_codecs::aper::AperCodec for #name {
            type Output = Self;

            fn decode(data: &mut asn1_codecs::aper::AperCodecData) -> Result<Self::Output, asn1_codecs::aper::AperCodecError> {
                let decoded = asn1_codecs::aper::decode::#decode_fn_name(data, #sz_lb, #sz_ub, #sz_ext)?;
                Ok(Self(decoded))
            }

            fn encode(&self, data: &mut asn1_codecs::aper::AperCodecData) -> Result<(), asn1_codecs::aper::AperCodecError> {
                asn1_codecs::aper::encode::#encode_fn_name(data, #sz_lb, #sz_ub, #sz_ext, &self.0, false)
            }
        }
    };

    tokens.into()
}
