use proc_macro2::TokenStream;
use quote::quote;
use syn::{LitBool, LitStr};

use crate::attrs::TyCodecParams;

pub(super) fn get_bounds_extensible_from_params(
    params: &TyCodecParams,
) -> (TokenStream, TokenStream, TokenStream) {
    get_bounds_extensible_from_lb_ub_ext(&params.lb, &params.ub, &params.ext)
}

pub(super) fn get_sz_bounds_extensible_from_params(
    params: &TyCodecParams,
) -> (TokenStream, TokenStream, TokenStream) {
    get_bounds_extensible_from_lb_ub_ext(&params.sz_lb, &params.sz_ub, &params.sz_ext)
}

fn get_bounds_extensible_from_lb_ub_ext(
    lb: &Option<LitStr>,
    ub: &Option<LitStr>,
    ext: &Option<LitBool>,
) -> (TokenStream, TokenStream, TokenStream) {
    let lb = if lb.is_some() {
        let lb = lb.as_ref().unwrap().value().parse::<i128>().unwrap();
        quote! {
            Some(#lb)
        }
    } else {
        quote! {
            None
        }
    };
    let ub = if ub.is_some() {
        let ub = ub.as_ref().unwrap().value().parse::<i128>().unwrap();
        quote! {
            Some(#ub)
        }
    } else {
        quote! {
            None
        }
    };
    let ext = if ext.is_some() {
        let ext = ext.as_ref();
        quote! {
            #ext
        }
    } else {
        quote! {
            false
        }
    };

    (lb, ub, ext)
}
