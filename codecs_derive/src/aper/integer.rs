//! `APER` Code generation for ASN.1 INTEGER Type

use quote::quote;

use crate::attrs::TyCodecParams;

pub(super) fn generate_aper_decode_for_asn_integer(
    ast: &syn::DeriveInput,
    params: &TyCodecParams,
) -> proc_macro::TokenStream {
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

    let lb = if params.lb.is_some() {
        let lb = params.lb.as_ref().unwrap().value().parse::<i128>().unwrap();
        quote! {
            Some(#lb)
        }
    } else {
        quote! {
            None
        }
    };
    let ub = if params.ub.is_some() {
        let ub = params.ub.as_ref().unwrap().value().parse::<i128>().unwrap();
        quote! {
            Some(#ub)
        }
    } else {
        quote! {
            None
        }
    };
    let ext = if params.ext.is_some() {
        let ext = params.ext.as_ref();
        quote! {
            #ext
        }
    } else {
        quote! {
            false
        }
    };

    let tokens = quote! {

        impl asn_codecs::aper::AperCodec for #name {
            type Output = Self;

            fn decode(data: &mut asn_codecs::aper::AperCodecData) -> Result<Self::Output, asn_codecs::aper::AperCodecError> {
                let decoded = asn_codecs::aper::decode::decode_integer(data, #lb, #ub, #ext)?;
                Ok(Self(decoded.0 as #ty))
            }
        }
    };

    tokens.into()
}
