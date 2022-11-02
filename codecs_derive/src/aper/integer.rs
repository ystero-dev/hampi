//! `APER` Code generation for ASN.1 INTEGER Type

use quote::quote;

use crate::{attrs::TyCodecParams, utils};

pub(super) fn generate_aper_codec_for_asn_integer(
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

    let (lb, ub, ext) = utils::get_bounds_extensible_from_params(params);

    let tokens = quote! {

        impl asn1_codecs::aper::AperCodec for #name {
            type Output = Self;

            fn aper_decode(data: &mut asn1_codecs::aper::AperCodecData) -> Result<Self::Output, asn1_codecs::aper::AperCodecError> {
                log::debug!(concat!("decode: ", stringify!(#name)));

                let decoded = asn1_codecs::aper::decode::decode_integer(data, #lb, #ub, #ext)?;
                Ok(Self(decoded.0 as #ty))
            }
            fn aper_encode(&self, data: &mut asn1_codecs::aper::AperCodecData) -> Result<(), asn1_codecs::aper::AperCodecError> {
                log::debug!(concat!("encode: ", stringify!(#name)));

                asn1_codecs::aper::encode::encode_integer(data, #lb, #ub, #ext, self.0 as i128, false)
            }
        }
    };

    tokens.into()
}
