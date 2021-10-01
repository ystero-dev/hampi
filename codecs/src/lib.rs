#![allow(dead_code)]
pub mod aper;

use syn;

#[derive(Default)]
pub struct Asn1CodecParams<T>
where
    T: Default,
{
    tag: Option<u32>,

    value_extensible: bool,
    size_extensible: bool,

    lb: Option<T>,
    ub: Option<T>,
    size_lb: Option<T>,
    size_ub: Option<T>,
}

impl<T: Default> Asn1CodecParams<T> {
    pub fn from_ast(_ast: &syn::DeriveInput) -> Self {
        Self::default()
    }
}
