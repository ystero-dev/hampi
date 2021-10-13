//! Module handling the Attributes of the `Struct`/`Enum` and their `Fields`/`Variants`
//!
//! A lot of this code is inspired from `serde_derive_internal` from the venerable `serde` crate.
use super::symbol::*;

#[derive(Debug, Default)]
pub(crate) struct CodecParams {
    pub(crate) ty: Option<syn::LitStr>,
    pub(crate) lb: Option<syn::LitInt>,
    pub(crate) ub: Option<syn::LitInt>,
    pub(crate) ext: Option<syn::LitBool>,
    pub(crate) sz_lb: Option<syn::LitInt>,
    pub(crate) sz_ub: Option<syn::LitInt>,
    pub(crate) sz_ext: Option<syn::LitBool>,
    pub(crate) key: Option<syn::LitInt>,
}

// Parse All the attributes of the Variant and generate CodecParams Struct
//
// The attributes of the `Struct` or `Enum` are parsed and a `CodecParams` structure is generated.
// This stucture will hold the values that will be used by the individual `decode` functions.
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
            Ok(syn::Meta::List(meta)) => {
                for nested in meta.nested.into_iter() {
                    match nested {
                        // parses #[asn(lb = 0)]
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.path == LB => {
                            match m.lit {
                                syn::Lit::Int(ref i) => {
                                    let lb = i.clone();
                                    codec_params.lb.replace(lb);
                                }
                                _ => errors.push(syn::Error::new_spanned(
                                    nested,
                                    "`lb` value should be an Integer Literal",
                                )),
                            }
                        }
                        // parses #[asn(ub = 0)]
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.path == UB => {
                            match m.lit {
                                syn::Lit::Int(ref i) => {
                                    let ub = i.clone();
                                    codec_params.ub.replace(ub);
                                }
                                _ => errors.push(syn::Error::new_spanned(
                                    nested,
                                    "`ub` value should be an Integer Literal",
                                )),
                            }
                        }
                        // parses #[asn(type = "CHOICE")]
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.path == TYPE => {
                            match m.lit {
                                syn::Lit::Str(ref s) => {
                                    let ty = s.clone();
                                    codec_params.ty.replace(ty);
                                }
                                _ => errors.push(syn::Error::new_spanned(
                                    nested,
                                    "`type` value should be an String Literal",
                                )),
                            }
                        }
                        // parses #[asn(extensible = true)]
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m))
                            if m.path == EXTENSIBLE =>
                        {
                            match m.lit {
                                syn::Lit::Bool(ref e) => {
                                    let ext = e.clone();
                                    codec_params.ext.replace(ext);
                                }
                                _ => errors.push(syn::Error::new_spanned(
                                    nested,
                                    "`extensible` value should be a Boolean Literal",
                                )),
                            }
                        }
                        // parses #[asn(sz_extensible = true)]
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m))
                            if m.path == SZ_EXTENSIBLE =>
                        {
                            match m.lit {
                                syn::Lit::Bool(ref se) => {
                                    let sz_ext = se.clone();
                                    codec_params.sz_ext.replace(sz_ext);
                                }
                                _ => errors.push(syn::Error::new_spanned(
                                    nested,
                                    "`extensible` value should be a Boolean Literal",
                                )),
                            }
                        }
                        // parses #[asn(sz_lb = 0)]
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.path == SZ_LB => {
                            match m.lit {
                                syn::Lit::Int(ref sz_lb) => {
                                    let sz_lb = sz_lb.clone();
                                    codec_params.sz_lb.replace(sz_lb);
                                }
                                _ => errors.push(syn::Error::new_spanned(
                                    nested,
                                    "`sz_lb` value should be an Int Literal",
                                )),
                            }
                        }
                        // parses #[asn(sz_ub = 0)]
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.path == SZ_UB => {
                            match m.lit {
                                syn::Lit::Int(ref sz_ub) => {
                                    let sz_ub = sz_ub.clone();
                                    codec_params.sz_ub.replace(sz_ub);
                                }
                                _ => errors.push(syn::Error::new_spanned(
                                    nested,
                                    "`sz_ub` value should be an Int Literal",
                                )),
                            }
                        }
                        // parses #[asn(sz_ub = 0)]
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.path == KEY => {
                            match m.lit {
                                syn::Lit::Int(ref key) => {
                                    let key = key.clone();
                                    codec_params.key.replace(key);
                                }
                                _ => errors.push(syn::Error::new_spanned(
                                    nested,
                                    "`key` value should be an Int Literal",
                                )),
                            }
                        }
                        _ => errors.push(syn::Error::new_spanned(
                            &nested,
                            "Unsupported attribute value. Attribute values should be of the form `a = b`"
                        )),
                    }
                }
            }
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
