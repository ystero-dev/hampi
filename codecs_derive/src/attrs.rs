//! Module handling the Attributes of the `Struct`/`Enum` and their `Fields`/`Variants`
//!
//! A lot of this code is inspired from `serde_derive_internal` from the venerable `serde` crate.
use super::symbol::*;

// `struct` or `enum` Attributes that are used by the Codec
#[derive(Debug, Default)]
pub(crate) struct TyCodecParams {
    // ASN Type for the Struct or Enum. Required
    pub(crate) ty: Option<syn::LitStr>,

    // Lower Bound for the Value (valid for Integers, Enums and Choice Index)
    pub(crate) lb: Option<syn::LitStr>,

    // Upper Bound for the Value (valid for Integers, Enums and Choice Index)
    pub(crate) ub: Option<syn::LitStr>,

    // Is the Value Extensible (Extension Marker is present in the definition.)
    pub(crate) ext: Option<syn::LitBool>,

    // Size Constraint Lower Bound.
    pub(crate) sz_lb: Option<syn::LitStr>,

    // Size Constraint Upper Bound.
    pub(crate) sz_ub: Option<syn::LitStr>,

    // Size Constraint Extensible
    pub(crate) sz_ext: Option<syn::LitBool>,

    // Number of Optional Fields (In ASN.1 SEQUENCE types.)
    pub(crate) optional_fields: Option<syn::LitInt>,

    // The actual 'attribute' from the Syntax tree from which this struct is generated. This will
    // be used mainly for error reporting inside the functions where this struct is passed.
    pub(crate) attr: Option<syn::Attribute>,
}

// Parse All the attributes of the Type and generate TyCodecParams Struct
//
// The attributes of the `Struct` or `Enum` are parsed and a `TyCodecParams` structure is generated.
// This stucture will hold the values that will be used by the individual `decode` functions.
pub(crate) fn parse_ty_meta_as_codec_params(
    attrs: &Vec<syn::Attribute>,
) -> Result<TyCodecParams, syn::Error> {
    let mut codec_params = TyCodecParams::default();

    let mut errors = vec![];
    for attr in attrs {
        if attr.path != ASN {
            errors.push(syn::Error::new_spanned(
                attr,
                format!("Unsupported attribute"),
            ));
            continue;
        }
        let _ = codec_params.attr.replace(attr.clone());
        match attr.parse_meta() {
            Ok(syn::Meta::List(meta)) => {
                for nested in meta.nested.into_iter() {
                    match nested {
                        // parses #[asn(lb = 0)]
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.path == LB => {
                            match m.lit {
                                syn::Lit::Str(ref i) => {
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
                                syn::Lit::Str(ref i) => {
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
                                syn::Lit::Str(ref sz_lb) => {
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
                                syn::Lit::Str(ref sz_ub) => {
                                    let sz_ub = sz_ub.clone();
                                    codec_params.sz_ub.replace(sz_ub);
                                }
                                _ => errors.push(syn::Error::new_spanned(
                                    nested,
                                    "`sz_ub` value should be an Int Literal",
                                )),
                            }
                        }
                        // parses #[asn(optional_fields = 1)]
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.path == OPTIONAL_FIELDS => {
                            match m.lit {
                                syn::Lit::Int(ref opt_flds) => {
                                    let opt_flds = opt_flds.clone();
                                    codec_params.optional_fields.replace(opt_flds);
                                }
                                _ => errors.push(syn::Error::new_spanned(
                                    nested,
                                    "`optional_fields` value should be an Int Literal",
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
                "Only attribute values of the form a = b are supported for 'asn' attribute.",
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

// Field or Variant Attributes used as Parameters by Codec
//
// We are using one common structure for both `struct` fields and `enum` variants.
#[derive(Debug, Default)]
pub(crate) struct FieldVarCodecParams {
    // Key used by this variant (Typically for an Enum)
    pub(crate) key: Option<syn::LitInt>,

    // Is the value from outside the "Extension" (ie. not from the Extension Root.)
    pub(crate) extended: Option<syn::LitBool>,

    // Optional Field Index
    pub(crate) optional_idx: Option<syn::LitInt>,

    // If this is a Key Field
    pub(crate) key_field: Option<syn::LitBool>,

    // The actual 'attribute' from the Syntax tree from which this struct is generated. This will
    // be used mainly for error reporting inside the functions where this struct is passed.
    pub(crate) attr: Option<syn::Attribute>,
}

// Parses attributes of the field (struct) or a variant (enum) to generate codec params.
pub(crate) fn parse_fld_meta_as_codec_params(
    attrs: &Vec<syn::Attribute>,
) -> Result<FieldVarCodecParams, syn::Error> {
    let mut codec_params = FieldVarCodecParams::default();

    let mut errors = vec![];
    for attr in attrs {
        if attr.path != ASN {
            errors.push(syn::Error::new_spanned(
                attr,
                format!("Unsupported attribute"),
            ));
            continue;
        }
        let _ = codec_params.attr.replace(attr.clone());
        match attr.parse_meta() {
            Ok(syn::Meta::List(meta)) => {
                for nested in meta.nested.into_iter() {
                    match nested {
                        // parses #[asn(key = <int>)]
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
                        // parses #[asn(extended = true)]
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.path == EXTENDED => {
                            match m.lit {
                                syn::Lit::Bool(ref ext) => {
                                    let ext = ext.clone();
                                    codec_params.extended.replace(ext);
                                }
                                _ => errors.push(syn::Error::new_spanned(
                                    nested,
                                    "`extended` value should be an Bool Literal",
                                )),
                            }
                        }
                        // parses #[asn(extended = true)]
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.path == OPTIONAL_IDX => {
                            match m.lit {
                                syn::Lit::Int(ref opt_idx) => {
                                    let opt_idx = opt_idx.clone();
                                    codec_params.optional_idx.replace(opt_idx);
                                }
                                _ => errors.push(syn::Error::new_spanned(
                                    nested,
                                    "`optional_idx` value should be an Integer Literal",
                                )),
                            }
                        }
                        // parses #[asn(key_field = true)]
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.path == KEY_FIELD => {
                            match m.lit {
                                syn::Lit::Bool(ref keyfld) => {
                                    let keyfld = keyfld.clone();
                                    codec_params.key_field.replace(keyfld);
                                }
                                _ => errors.push(syn::Error::new_spanned(
                                    nested,
                                    "`extended` value should be an Bool Literal",
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
                "Only attribute values of the form a = b are supported for 'asn' attribute.",
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
