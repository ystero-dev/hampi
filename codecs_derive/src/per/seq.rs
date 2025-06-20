//! `APER` Code generation for ASN.1 `SEQUENCE` type

use quote::quote;

use crate::attrs::{parse_fld_meta_as_codec_params, TyCodecParams};

struct FieldTokens {
    decode_tokens: Vec<proc_macro2::TokenStream>,
    encode_tokens: Vec<proc_macro2::TokenStream>,
    hdr_encode_tokens: Vec<proc_macro2::TokenStream>,
}

pub(super) fn generate_per_codec_for_asn_sequence(
    ast: &syn::DeriveInput,
    params: &TyCodecParams,
    aligned: bool,
) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let (codec_path, codec_encode_fn, codec_decode_fn, ty_encode_path, ty_decode_path) = if aligned
    {
        (
            quote!(asn1_codecs::aper::AperCodec),
            quote!(aper_encode),
            quote!(aper_decode),
            quote!(asn1_codecs::aper::encode::encode_sequence_header),
            quote!(asn1_codecs::aper::decode::decode_sequence_header),
        )
    } else {
        (
            quote!(asn1_codecs::uper::UperCodec),
            quote!(uper_encode),
            quote!(uper_decode),
            quote!(asn1_codecs::uper::encode::encode_sequence_header),
            quote!(asn1_codecs::uper::decode::decode_sequence_header),
        )
    };

    let ext = params.ext.as_ref();
    let opt_count = if params.optional_fields.is_some() {
        params.optional_fields.as_ref().unwrap().clone()
    } else {
        syn::LitInt::new("0", proc_macro2::Span::call_site())
    };

    let aligned = syn::LitBool::new(aligned, proc_macro2::Span::call_site());

    let field_tokens = generate_seq_field_codec_tokens_using_attrs(
        ast,
        codec_encode_fn.clone(),
        codec_decode_fn.clone(),
    );
    if field_tokens.is_err() {
        return field_tokens.err().unwrap().to_compile_error().into();
    }
    let field_tokens = field_tokens.unwrap();
    let fld_decode_tokens = field_tokens.decode_tokens;
    let hdr_encode_tokens = field_tokens.hdr_encode_tokens;
    let fld_encode_tokens = field_tokens.encode_tokens;

    let tokens = quote! {
        impl #codec_path for #name {
            type Output = Self;

            fn #codec_decode_fn(data: &mut asn1_codecs::PerCodecData) -> Result<Self::Output, asn1_codecs::PerCodecError> {
                log::trace!(concat!("decode: ", stringify!(#name)));

                let (bitmap, extensions_present) = #ty_decode_path(data, #ext, #opt_count)?;
                if !extensions_present {
                    Ok(Self{#(#fld_decode_tokens)*})
                } else {
                    let ty = Self {
                        #(#fld_decode_tokens)*
                    };
                    asn1_codecs::decode_sequence_extensions_skip_bits(data, #aligned)?;
                    Ok(ty)
                }
            }

            fn #codec_encode_fn(&self, data: &mut asn1_codecs::PerCodecData) -> Result<(), asn1_codecs::PerCodecError> {
                log::trace!(concat!("encode: ", stringify!(#name)));

                let mut bitmap = bitvec::bitvec![u8, bitvec::prelude::Msb0; 0; #opt_count];

                #(#hdr_encode_tokens)*

                #ty_encode_path(data, #ext, &bitmap, false)?;

                #(#fld_encode_tokens)*

                Ok(())
            }
        }
    };

    tokens.into()
}

fn generate_seq_field_codec_tokens_using_attrs(
    ast: &syn::DeriveInput,
    codec_encode_fn: proc_macro2::TokenStream,
    codec_decode_fn: proc_macro2::TokenStream,
) -> Result<FieldTokens, syn::Error> {
    let mut decode_tokens = vec![];
    let mut encode_tokens = vec![];
    let mut hdr_encode_tokens = vec![];

    let mut errors: Vec<syn::Error> = vec![];
    if let syn::Data::Struct(ref data) = ast.data {
        if let syn::Fields::Named(ref fields) = data.fields {
            for field in &fields.named {
                let codec_params = parse_fld_meta_as_codec_params(&field.attrs);
                match codec_params {
                    Err(e) => errors.push(e),
                    Ok(cp) => {
                        let field_type = get_field_type(field);
                        if field_type.ty.is_none() {
                            errors.push(syn::Error::new_spanned(
                                field,
                                "Field Type is not in supported Format!",
                            ));
                            continue;
                        } else {
                            let ty_ident = field_type.ty.unwrap();
                            let optional = field_type.is_optional;
                            let fld_decode_tokens = if optional {
                                let optional_idx = cp.optional_idx.as_ref();

                                match optional_idx {
                                    None => {
                                        errors.push(syn::Error::new_spanned(
                                            field,
                                            "Optional Field without Optional Index.",
                                        ));
                                        // Just pass empty quote
                                        quote! {}
                                    }
                                    Some(optidx) => {
                                        quote! {
                                            {
                                            let present = bitmap[#optidx];
                                            if present {
                                                Some(#ty_ident::#codec_decode_fn(data)?)
                                            } else {
                                                None
                                            }
                                            }
                                        }
                                    }
                                }
                            } else {
                                let key_field = cp.key_field.as_ref();
                                let is_key_field = if let Some(kf) = key_field {
                                    kf.value()
                                } else {
                                    false
                                };

                                if !is_key_field {
                                    quote! {
                                        {
                                        #ty_ident::#codec_decode_fn(data)?
                                        }
                                    }
                                } else {
                                    quote! {
                                        {
                                        let value = #ty_ident::#codec_decode_fn(data)?;
                                        let _ = data.set_key(value.0 as i128);
                                        value
                                        }
                                    }
                                }
                            };

                            let id = field.ident.as_ref().unwrap();
                            let field_encode_token = if optional {
                                quote! {
                                    if self.#id.is_some() {
                                        let #id = self.#id.as_ref().unwrap();
                                        #id.#codec_encode_fn(data)?;
                                    } else {
                                    }
                                }
                            } else {
                                quote! {
                                    self.#id.#codec_encode_fn(data)?;
                                }
                            };
                            let header_encode_token = if optional {
                                let optional_idx = cp.optional_idx.as_ref();
                                quote! {
                                    if self.#id.is_some() {
                                        bitmap.set(#optional_idx, true);
                                    } else {
                                    }
                                }
                            } else {
                                quote! {}
                            };
                            let field_decode_token = quote! { #id: #fld_decode_tokens, };
                            decode_tokens.push(field_decode_token);
                            encode_tokens.push(field_encode_token);
                            hdr_encode_tokens.push(header_encode_token);
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
        Ok(FieldTokens {
            decode_tokens,
            hdr_encode_tokens,
            encode_tokens,
        })
    }
}

struct StructFieldType {
    ty: Option<syn::Ident>,
    is_optional: bool,
}

fn get_field_type(field: &syn::Field) -> StructFieldType {
    fn field_is_optional(field: &syn::Field) -> bool {
        if let syn::Type::Path(ref typepath) = field.ty {
            typepath.path.leading_colon.is_none()
                && typepath.path.segments.len() == 1
                && typepath.path.segments.iter().next().unwrap().ident == "Option"
        } else {
            false
        }
    }

    let is_optional = field_is_optional(field);

    let ty = if is_optional {
        if let syn::Type::Path(ref tp) = field.ty {
            let type_params = &tp.path.segments.iter().next().unwrap().arguments;
            match type_params {
                syn::PathArguments::AngleBracketed(params) => {
                    let generic_args = params.args.iter().next().unwrap();
                    if let syn::GenericArgument::Type(syn::Type::Path(tpinner)) = generic_args {
                        Some(tpinner.path.segments.iter().next().unwrap().ident.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    } else if let syn::Type::Path(ref tp) = field.ty {
        Some(tp.path.segments.iter().next().unwrap().ident.clone())
    } else {
        None
    };

    StructFieldType { ty, is_optional }
}
