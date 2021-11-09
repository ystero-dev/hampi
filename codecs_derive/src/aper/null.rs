//! Handling of ASN.1 NULL Type

use quote::quote;

use crate::attrs::TyCodecParams;

pub(super) fn generate_aper_decode_for_asn_null(
    ast: &syn::DeriveInput,
    _params: &TyCodecParams,
) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let tokens = quote! {

        impl asn1_codecs::aper::AperCodec for #name {

            type Output = Self;

            fn decode(_data: &mut asn1_codecs::aper::AperCodecData) -> Result<Self::Output, asn1_codecs::aper::AperCodecError> {
                Ok(Self{})
            }
        }
    };

    tokens.into()
}
