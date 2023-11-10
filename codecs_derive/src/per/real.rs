//! Handling of ASN.1 REAL Type

use quote::quote;

use crate::attrs::TyCodecParams;

pub(super) fn generate_per_codec_for_asn_real(
    ast: &syn::DeriveInput,
    _params: &TyCodecParams,
    aligned: bool,
) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let (codec_path, codec_encode_fn, codec_decode_fn, ty_encode_path, ty_decode_path) = if aligned {
        (
            quote!(asn1_codecs::aper::AperCodec),
            quote!(aper_encode),
            quote!(aper_decode),
            quote!(asn1_codecs::aper::encode::encode_real),
            quote!(asn1_codecs::aper::decode::decode_real),
        )
    } else {
        (
            quote!(asn1_codecs::uper::UperCodec),
            quote!(uper_encode),
            quote!(uper_decode),
            quote!(asn1_codecs::uper::encode::encode_real),
            quote!(asn1_codecs::uper::decode::decode_real),
        )
    };
    let tokens = quote! {

        impl #codec_path for #name {

            type Output = Self;

            fn #codec_decode_fn(data: &mut asn1_codecs::PerCodecData) -> Result<Self::Output, asn1_codecs::PerCodecError> {
                log::trace!(concat!("decode: ", stringify!(#name)));

                let value = #ty_decode_path(data)?;
                Ok(Self(value))
            }

            fn #codec_encode_fn(&self, data: &mut asn1_codecs::PerCodecData) -> Result<(), asn1_codecs::PerCodecError> {
                log::trace!(concat!("encode: ", stringify!(#name)));

                #ty_encode_path(data, self.0)
            }
        }
    };

    tokens.into()
}
