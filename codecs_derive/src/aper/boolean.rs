//! `APER` Code generation for ASN.1 Boolean Type

use proc_macro::TokenStream;
use quote::quote;

use crate::attrs::TyCodecParams;

pub(super) fn generate_aper_decode_for_asn_boolean(
    ast: &syn::DeriveInput,
    _params: &TyCodecParams,
) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let tokens = quote! {

        impl asn_codecs::aper::AperCodec for #name {
            type Output = Self;

            fn decode(data: &mut asn_codecs::aper::AperCodecData) -> Result<Self::Output, asn_codecs::aper::AperCodecError> {

                let value = asn_codecs::aper::decode::decode_bool(data)?;
                Ok(Self(value))
            }
        }
    };

    TokenStream::from(tokens)
}
