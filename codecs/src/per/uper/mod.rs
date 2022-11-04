#![allow(dead_code)]
//! ASN.1 Un-aligned PER Codec

pub mod encode;

pub mod decode;

/// Trait representing an 'APER Codec'.
///
///
/// This 'trait' is to be derived by any `struct` or `enum` representing an ASN.1 Type.
pub trait UperCodec {
    type Output;

    fn uper_decode(data: &mut crate::PerCodecData) -> Result<Self::Output, crate::PerCodecError>;

    fn uper_encode(&self, _data: &mut crate::PerCodecData) -> Result<(), crate::PerCodecError> {
        todo!();
    }
}
