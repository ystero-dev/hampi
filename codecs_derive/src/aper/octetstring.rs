//! `APER` Code generation for ASN.1 OCTET STRING Type

use quote::quote;

use crate::attrs::TyCodecParams;

pub(super) fn generate_aper_decode_for_asn_octetstring(
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
                let decoded = asn1_codecs::aper::decode::decode_octetstring(data, #sz_lb, #sz_ub, #sz_ext)?;
                Ok(Self(decoded))
            }
        }
    };

    tokens.into()
}
