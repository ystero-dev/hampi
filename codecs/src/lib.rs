#![allow(dead_code)]
pub mod aper;

use proc_macro2::TokenStream;
use syn;

/// ASN.1 Codec Parameters
///
/// These parameters are used by the Codec to `encode` or `decode` a given `struct`, `enum` or a
/// `field` or a `variant`. The parameters are typically generated from the constraints on the type
/// and passed as `attributes` to the `struct`/`enum` or `variant`/`field`.
#[derive(Default)]
pub struct Asn1CodecParams<T>
where
    T: Default,
{
    tag: Option<u32>,

    value_extensible: bool,
    size_extensible: bool,
    open_type: bool,

    lb: Option<T>,
    ub: Option<T>,
    size_lb: Option<T>,
    size_ub: Option<T>,
}

impl<T: Default> Asn1CodecParams<T> {
    /// Returns a `TokenStream` for the `Asn1CodecParams` struct.
    ///
    ///
    pub fn from_container_ast(ast: &syn::DeriveInput) -> TokenStream {
        for attr in &ast.attrs {
            if attr.path.is_ident("asn1") {}
        }
        TokenStream::new()
    }
}
