//! `APER` Code generation for ASN.1 `SEQUENCE` type

use quote::quote;

use crate::attrs::{parse_fld_meta_as_codec_params, TyCodecParams};

pub(super) fn generate_per_codec_for_asn_open_type(
    ast: &syn::DeriveInput,
    _params: &TyCodecParams,
    aligned: bool,
) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let (
        codec_path,
        codec_new_perdata_path,
        codec_encode_fn,
        codec_decode_fn,
        ty_encode_path,
        ty_decode_path,
    ) = if aligned {
        (
            quote!(asn1_codecs::aper::AperCodec),
            quote!(asn1_codecs::PerCodecData::new_aper),
            quote!(aper_encode),
            quote!(aper_decode),
            quote!(asn1_codecs::aper::encode::encode_length_determinent),
            quote!(asn1_codecs::aper::decode::decode_length_determinent),
        )
    } else {
        (
            quote!(asn1_codecs::uper::UperCodec),
            quote!(asn1_codecs::PerCodecData::new_uper),
            quote!(uper_encode),
            quote!(uper_decode),
            quote!(asn1_codecs::uper::encode::encode_length_determinent),
            quote!(asn1_codecs::uper::decode::decode_length_determinent),
        )
    };
    let variant_tokens = generate_open_type_variant_tokens_using_attrs(
        ast,
        codec_encode_fn.clone(),
        codec_decode_fn.clone(),
    );
    if variant_tokens.is_err() {
        return variant_tokens.err().unwrap().to_compile_error().into();
    }
    let (variant_decode_tokens, variant_encode_tokens) = variant_tokens.unwrap();

    let encode_tokens = if !variant_encode_tokens.is_empty() {
        quote! {
                let mut inner = #codec_new_perdata_path();
                let _ = match self {
                    #(#variant_encode_tokens)*
                };
                let length = inner.length_in_bytes();
                let _ = #ty_encode_path(data, None, None, false, length)?;
                data.append_aligned(&mut inner);
                Ok(())
        }
    } else {
        quote! {
            Ok(())
        }
    };

    let tokens = quote! {
        impl #codec_path for #name {
            type Output = Self;

            fn #codec_decode_fn(data: &mut asn1_codecs::PerCodecData) -> Result<Self::Output, asn1_codecs::PerCodecError> {
                log::trace!(concat!("decode: ", stringify!(#name)));

                let length = #ty_decode_path(data, None, None, false)?;

                log::trace!("open type: decoded length: {}", length);

                if data.get_key().is_none() {
                    return Err(asn1_codecs::PerCodecError::new(
                            asn1_codecs::PerCodecErrorCause::Generic,
                            "Decoding OPEN Type, but `key` is not determined!"));
                }

                let key = data.get_key().unwrap();

                let result = match key {
                    #(#variant_decode_tokens)*
                    _ => Err(asn1_codecs::PerCodecError::new(
                            asn1_codecs::PerCodecErrorCause::Generic,
                            format!("Key {} Not Found", key).as_str()))
                }?;
                data.decode_align()?;
                Ok(result)
            }

            fn #codec_encode_fn(&self, data: &mut asn1_codecs::PerCodecData) -> Result<(), asn1_codecs::PerCodecError> {
                log::trace!(concat!("encode: ", stringify!(#name)));

                #encode_tokens
            }
        }
    };

    tokens.into()
}

fn generate_open_type_variant_tokens_using_attrs(
    ast: &syn::DeriveInput,
    codec_encode_fn: proc_macro2::TokenStream,
    codec_decode_fn: proc_macro2::TokenStream,
) -> Result<(Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>), syn::Error> {
    let mut decode_tokens = vec![];
    let mut encode_tokens = vec![];

    let mut errors = vec![];
    if let syn::Data::Enum(ref data) = ast.data {
        for variant in &data.variants {
            let codec_params = parse_fld_meta_as_codec_params(&variant.attrs);
            match codec_params {
                Err(e) => errors.push(e),
                Ok(cp) => {
                    let key = cp.key.as_ref();
                    if key.is_none() {
                        errors.push(syn::Error::new_spanned(
                        variant,
                        "Missing Key for the variant. Please provide `#[asn(key = <int>)]` attribute.",
                    ));
                        continue;
                    }
                    let variant_ident = &variant.ident;
                    if let syn::Fields::Unnamed(ref fields) = variant.fields {
                        if fields.unnamed.len() == 1 {
                            let unnamed = &fields.unnamed.first();
                            let unnamed = unnamed.as_ref().unwrap();
                            let ty = &unnamed.ty;
                            let variant_decode_token = quote! {
                                #key => Ok(Self::#variant_ident(#ty::#codec_decode_fn(data)?)),
                            };
                            let variant_encode_token = quote! {
                                Self::#variant_ident(ref v) => v.#codec_encode_fn(&mut inner)?,
                            };
                            decode_tokens.push(variant_decode_token);
                            encode_tokens.push(variant_encode_token);
                        } else {
                            errors.push(syn::Error::new_spanned(
                                variant,
                                "Unsupported variant type".to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }

    if let Some((first, others)) = errors.split_first_mut() {
        for e in others {
            first.combine(e.clone())
        }
        Err(first.clone())
    } else {
        Ok((decode_tokens, encode_tokens))
    }
}
