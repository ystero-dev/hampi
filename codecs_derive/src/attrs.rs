use quote::ToTokens;

use super::symbol::*;

#[derive(Debug, Default)]
pub(crate) struct CodecParams {
    pub(crate) ty: Option<syn::LitStr>,
    pub(crate) lb: Option<syn::LitInt>,
    pub(crate) ub: Option<syn::LitInt>,
    //extensible: Option<Literal>,
}

pub(crate) fn parse_variant_meta_as_codec_params(
    attrs: &Vec<syn::Attribute>,
) -> Result<CodecParams, syn::Error> {
    let mut codec_params = CodecParams::default();

    let mut errors = vec![];
    for attr in attrs {
        if attr.path != ASN {
            errors.push(syn::Error::new_spanned(
                attr,
                format!("Unsupported attribute"),
            ));
            continue;
        }
        match attr.parse_meta() {
            Ok(syn::Meta::List(meta)) => for nested in meta.nested.into_iter() {},
            Ok(other) => errors.push(syn::Error::new_spanned(
                other,
                "Only attributes of the type a = b are supported.",
            )),
            Err(error) => errors.push(error),
        }
    }
    if let Some((first, others)) = errors.split_first_mut() {
        for e in others {
            first.combine(e.clone())
        }
        Err(first.clone())
    } else {
        Ok(codec_params)
    }
}

pub(crate) fn get_asn_meta_items(
    attr: &syn::Attribute,
) -> Result<Vec<syn::NestedMeta>, syn::Error> {
    if attr.path != ASN {
        Ok(Vec::new())
    } else {
        match attr.parse_meta() {
            Ok(syn::Meta::List(meta)) => Ok(meta.nested.into_iter().collect()),
            _ => Err(syn::Error::new_spanned(
                attr.into_token_stream(),
                "Only attributes of the type a = b are supported.",
            )),
        }
    }
}

pub(crate) fn get_codec_params_from_meta_items(
    attrs: &Vec<syn::Attribute>,
) -> Result<CodecParams, ()> {
    let mut codec_params = CodecParams::default();

    let meta_items = attrs
        .iter()
        .flat_map(|attr| get_asn_meta_items(attr))
        .flatten()
        .collect::<Vec<syn::NestedMeta>>();

    for item in meta_items {
        match item {
            syn::NestedMeta::Meta(syn::Meta::NameValue(m)) if m.path == LB => match m.lit {
                syn::Lit::Int(ref i) => {
                    let lb = i.clone();
                    codec_params.lb.replace(lb);
                }
                _ => (),
            },
            syn::NestedMeta::Meta(syn::Meta::NameValue(m)) if m.path == UB => match m.lit {
                syn::Lit::Int(ref i) => {
                    let ub = i.clone();
                    codec_params.ub.replace(ub);
                }
                _ => (),
            },
            _ => (),
        }
    }
    Ok(codec_params)
}
