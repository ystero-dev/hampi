use proc_macro::TokenStream;
use quote::quote;

use super::attrs::CodecParams;

pub(crate) fn generate_decode(
    ast: &syn::DeriveInput,
    params: &CodecParams,
) -> proc_macro::TokenStream {
    let ty = params.ty.as_ref().unwrap();
    match ty.value().as_str() {
        "CHOICE" => generate_aper_decode_for_asn_choice(ast, params),
        _ => syn::Error::new_spanned(ty.clone(), "This ASN.1 Type is not supported.")
            .to_compile_error()
            .into(),
    }
}

fn generate_aper_decode_for_asn_choice(
    ast: &syn::DeriveInput,
    params: &CodecParams,
) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let lb = params.lb.as_ref();
    let ub = params.ub.as_ref();
    let ext = params.ext.as_ref();

    let tokens = quote! {

        impl asn_codecs::aper::AperCodec for #name {
            type Output = Self;

            fn decode(data: &mut asn_codecs::aper::AperCodecData) -> Result<Self::Output, asn_codecs::aper::AperCodecError> {

                asn_codecs::aper::decode_choice_idx(data, #lb, #ub, #ext);

                Ok(Self{})
            }
        }
    };

    TokenStream::from(tokens)
}
