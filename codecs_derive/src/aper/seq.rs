//! `APER` Code generation for ASN.1 `SEQUENCE` type

use quote::quote;

use crate::attrs::{parse_fld_meta_as_codec_params, TyCodecParams};

pub(super) fn generate_aper_decode_for_asn_sequence(
    ast: &syn::DeriveInput,
    params: &TyCodecParams,
) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let ext = params.ext.as_ref();
    let opt_count = if params.optional_fields.is_some() {
        params.optional_fields.as_ref().unwrap().clone()
    } else {
        syn::LitInt::new("0", proc_macro2::Span::call_site())
    };

    let fld_tokens = generate_seq_field_tokens_using_attrs(ast);
    if fld_tokens.is_err() {
        return fld_tokens.err().unwrap().to_compile_error().into();
    }
    let fld_tokens = fld_tokens.unwrap();

    let tokens = quote! {
        impl asn1_codecs::aper::AperCodec for #name {
            type Output = Self;

            fn decode(data: &mut asn1_codecs::aper::AperCodecData) -> Result<Self::Output, asn1_codecs::aper::AperCodecError> {
                let (bitmap, _extensions_present) = asn1_codecs::aper::decode::decode_sequence_header(data, #ext, #opt_count)?;
                Ok(Self{#(#fld_tokens)*})

            }
        }
    };

    tokens.into()
}

fn generate_seq_field_tokens_using_attrs(
    ast: &syn::DeriveInput,
) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
    let mut tokens = vec![];

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
                            let decode_tokens = if optional {
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
                                                Some(#ty_ident::decode(data)?)
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
                                        #ty_ident::decode(data)?
                                        }
                                    }
                                } else {
                                    quote! {
                                        {
                                        let value = #ty_ident::decode(data)?;
                                        let _ = data.set_key(value.0 as i128);
                                        value
                                        }
                                    }
                                }
                            };

                            let id = field.ident.as_ref().unwrap();
                            let field_token = quote! { #id: #decode_tokens, };
                            tokens.push(field_token);
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
        Ok(tokens)
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
