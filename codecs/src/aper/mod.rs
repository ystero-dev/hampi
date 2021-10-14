#![allow(dead_code)]
//! ASN.1 Aligned PER Codec

pub mod error;

pub use error::Error as AperCodecError;

mod decode;

pub use decode::*;

/// Trait representing an 'APER Codec'.
///
/// This 'crate' is to be derived by any `struct` or `enum` representing an ASN.1 Type.
pub trait AperCodec {
    type Output;

    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError>;
}

use bitvec::prelude::*;

/// Structure representing an APER Codec.
///
/// While En(De)coding ASN.1 Types using the APER encoding scheme, the encoded data is stored in a
/// `BitVec`.
#[derive(Default)]
pub struct AperCodecData {
    bits: BitVec<Msb0, u8>,
    offset: usize,
}

impl AperCodecData {
    pub fn new() -> Self {
        Self::default()
    }

    fn align(&mut self) -> () {
        let remaining = 8 - (self.offset & 0x7 as usize);
        let mut bv = bitvec![Msb0, u8; 0; remaining];
        self.bits.append(&mut bv);
        self.offset += remaining;
    }

    pub fn to_i32(
        &mut self,
        extension: bool,
        _lb: Option<i32>,
        _ub: Option<i32>,
    ) -> Result<i32, AperCodecError> {
        // Extension bit is present
        if extension {
            // FIXME: Will panic if offset > BitVec length.
            let extension_bit = self.bits.get(self.offset).unwrap() == true;
            if extension_bit {
                self.offset += 1;
                return self.decode_unconstrained_i32();
            };
        }
        Ok(0_i32)
    }

    fn decode_unconstrained_i32(&mut self) -> Result<i32, AperCodecError> {
        let out = 0 as i32;
        Ok(out)
    }
}
