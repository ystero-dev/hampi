//! Handling of ASN.1 NULL Type

use quote::quote;

use crate::attrs::TyCodecParams;

pub(super) fn generate_aper_codec_for_asn_object_identifier(
    ast: &syn::DeriveInput,
    _params: &TyCodecParams,
) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let tokens = quote! {

        impl asn1_codecs::aper::AperCodec for #name {

            type Output = Self;

            fn decode(_data: &mut asn1_codecs::aper::AperCodecData) -> Result<Self::Output, asn1_codecs::aper::AperCodecError> {
                Err(asn1_codecs::aper::AperCodecError::new("Object Identifier Decode Not Supported!"))
            }
            fn encode(&self, _data: &mut asn1_codecs::aper::AperCodecData) -> Result<(), asn1_codecs::aper::AperCodecError> {
                Err(asn1_codecs::aper::AperCodecError::new("Object Identifier Encode Not Supported!"))
            }
        }
    };

    tokens.into()
}
