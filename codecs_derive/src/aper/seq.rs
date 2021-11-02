//! `APER` Code generation for ASN.1 `SEQUENCE` type

use quote::quote;

use crate::attrs::TyCodecParams;

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

    let fld_tokens = generate_seq_field_tokens_using_attrs(ast, params);
    if fld_tokens.is_err() {
        return fld_tokens.err().unwrap().to_compile_error().into();
    }
    let fld_tokens = fld_tokens.unwrap();

    let tokens = quote! {
        impl asn_codecs::aper::AperCodec for #name {
            type Output = Self;

            fn decode(data: &mut asn_codecs::aper::AperCodecData) -> Result<Self::Output, asn_codecs::aper::AperCodecError> {
                let (bitmap, extensions_present) = asn_codecs::aper::decode::decode_sequence_header(data, #ext, #opt_count);


                Ok(Self{#(#fld_tokens)*})

            }
        }
    };

    tokens.into()
}

fn generate_seq_field_tokens_using_attrs(
    ast: &syn::DeriveInput,
    params: &TyCodecParams,
) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
    let mut tokens = vec![];

    let mut errors: Vec<syn::Error> = vec![];
    if let syn::Data::Struct(ref data) = ast.data {
        if let syn::Fields::Named(ref fields) = data.fields {
            for field in &fields.named {
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
                        quote! {
                            {
                            let present = bitmap.get(0);
                            if present {
                                Some(#ty_ident::decode(data)?)
                            } else {
                                None
                            }
                            }
                        }
                    } else {
                        quote! { #ty_ident::decode(data)? }
                    };

                    let id = field.ident.as_ref().unwrap();
                    let field_token = quote! { #id: #decode_tokens, };
                    tokens.push(field_token);
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
                    if let syn::GenericArgument::Type(ty) = generic_args {
                        if let syn::Type::Path(tpinner) = ty {
                            Some(tpinner.path.segments.iter().next().unwrap().ident.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    } else {
        if let syn::Type::Path(ref tp) = field.ty {
            Some(tp.path.segments.iter().next().unwrap().ident.clone())
        } else {
            None
        }
    };

    StructFieldType { ty, is_optional }
}
