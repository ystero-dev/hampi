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

    pub fn from_u8(bits: &[u8]) -> Self {
        Self {
            bits: BitSlice::<Msb0, _>::from_slice(bits).unwrap().to_bitvec(),
            offset: 0,
        }
    }

    fn decode_align(&mut self) -> Result<(), AperCodecError> {
        let remaining = 8 - (self.offset & 0x7 as usize);
        if !self.bits[self.offset..self.offset + remaining]
            .iter()
            .all(|b| b == false)
        {
            Err(AperCodecError::new(
                format!(
                    "{} Padding bits at Offset {} not all '0'.",
                    remaining, self.offset,
                )
                .as_str(),
            ))
        } else {
            self.offset += remaining;
            Ok(())
        }
    }

    fn align(&mut self) -> () {
        let remaining = 8 - (self.offset & 0x7 as usize);
        let mut bv = bitvec![Msb0, u8; 0; remaining];
        self.bits.append(&mut bv);
        self.offset += remaining;
    }

    fn decode_bits(&mut self, bits: usize) -> Result<i128, AperCodecError> {
        let remaining = self.bits.len() - self.offset;
        if remaining < bits {
            Err(AperCodecError::new(
                format!(
                    "AperCodec:DecodeError:Requested Bits to decode {}, Remaining bits {}",
                    bits, remaining
                )
                .as_str(),
            ))
        } else {
            eprintln!("offset: {}, bits: {}", self.offset, bits);
            let value = self.bits[self.offset..self.offset + bits].load_be::<u16>() as i128;
            eprintln!("value: {:#?}", value);
            Ok(value)
        }
    }

    pub fn advance(&mut self, bits: usize) -> () {
        let offset = self.offset + bits;
        if offset > self.bits.len() {
            self.offset = self.bits.len()
        } else {
            self.offset = offset
        }
    }
}
