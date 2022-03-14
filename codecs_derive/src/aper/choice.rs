//! `APER` Code generation for ASN.1 Choice Type

use proc_macro::TokenStream;
use quote::quote;

use crate::attrs::{parse_fld_meta_as_codec_params, TyCodecParams};

pub(super) fn generate_aper_codec_for_asn_choice(
    ast: &syn::DeriveInput,
    params: &TyCodecParams,
) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let lb = params.lb.as_ref().unwrap().value().parse::<i128>().unwrap();
    let ub = params.ub.as_ref().unwrap().value().parse::<i128>().unwrap();
    let ext = params.ext.as_ref();

    let variant_tokens = generate_choice_variant_decode_tokens_using_attrs(ast, lb, ub, ext);
    if variant_tokens.is_err() {
        return variant_tokens.err().unwrap().to_compile_error().into();
    }
    let (variant_decode_tokens, variant_encode_tokens) = variant_tokens.unwrap();

    let tokens = quote! {

        impl asn1_codecs::aper::AperCodec for #name {
            type Output = Self;

            fn decode(data: &mut asn1_codecs::aper::AperCodecData) -> Result<Self::Output, asn1_codecs::aper::AperCodecError> {

                let (idx, extended) = asn1_codecs::aper::decode::decode_choice_idx(data, #lb, #ub, #ext)?;
                if !extended {
                    match idx {
                        #(#variant_decode_tokens)*
                        _ => Err(asn1_codecs::aper::AperCodecError::new(format!("Index {} is not a valid Choice Index", idx).as_str()))
                    }
                } else {
                    Err(asn1_codecs::aper::AperCodecError::new("CHOICE Additions not supported yet."))
                }
            }

            fn encode(&self, data: &mut asn1_codecs::aper::AperCodecData) -> Result<(), asn1_codecs::aper::AperCodecError> {
                match self {
                    #(#variant_encode_tokens)*
                }
            }
        }
    };

    TokenStream::from(tokens)
}

fn generate_choice_variant_decode_tokens_using_attrs(
    ast: &syn::DeriveInput,
    lb: i128,
    ub: i128,
    ext: Option<&syn::LitBool>,
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
                    let _extended = cp.extended.as_ref();
                    let variant_ident = &variant.ident;
                    if let syn::Fields::Unnamed(ref fields) = variant.fields {
                        if fields.unnamed.len() == 1 {
                            let ty = &fields.unnamed.first().as_ref().unwrap().ty;
                            let variant_decode_token = quote! {
                                #key => Ok(Self::#variant_ident(#ty::decode(data)?)),
                            };
                            let variant_encode_token = quote! {
                                Self::#variant_ident(ref v) => {
                                    asn1_codecs::aper::encode::encode_choice_idx(data, #lb, #ub, #ext, #key)?;
                                    v.encode(data)
                                }
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
