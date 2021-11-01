#![allow(dead_code)]
//! ASN.1 Aligned PER Codec

pub mod error;

pub use error::Error as AperCodecError;

pub mod decode;

//pub use decode::*;

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
    /// Default `AperCodecData`
    pub fn new() -> Self {
        Self::default()
    }

    /// Create Our `AperCodecData` Structure from a slice of u8
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self {
            bits: BitSlice::<_, _>::from_slice(bytes).unwrap().to_bitvec(),
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

    fn decode_bool(&mut self) -> Result<bool, AperCodecError> {
        if self.bits.len() == self.offset {
            return Err(AperCodecError::new(
                format!(
                    "AperCodec:DecodeError:End of Bitstream reached while trying to decode bool."
                )
                .as_str(),
            ));
        }
        let bit = *self.bits.get(self.offset).as_deref().unwrap();
        let _ = self.advance_maybe_err(1, true)?;

        Ok(bit)
    }

    fn decode_bits_as_integer(&mut self, bits: usize) -> Result<i128, AperCodecError> {
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
            let value = if bits == 0 {
                0 as i128
            } else {
                self.bits[self.offset..self.offset + bits].load_be::<u128>() as i128
            };
            eprintln!("value: {:#?}", value);
            self.advance_maybe_err(bits, false)?;
            Ok(value)
        }
    }

    fn advance_maybe_err(&mut self, bits: usize, ignore: bool) -> Result<(), AperCodecError> {
        let offset = self.offset + bits;
        if offset > self.bits.len() {
            if ignore {
                self.offset = self.bits.len()
            } else {
                let remaining = self.bits.len() - self.offset;
                return Err(AperCodecError::new(
                    format!(
                        "AperCodec:DecodeError:Requested Bits to advance {}, Remaining bits {}",
                        bits, remaining
                    )
                    .as_str(),
                ));
            }
        } else {
            self.offset = offset
        }
        Ok(())
    }

    fn get_bit(&self) -> Result<bool, AperCodecError> {
        if self.offset >= self.bits.len() {
            return Err(AperCodecError::new(
                format!(
                    "AperCodec:GetBitError:Requested Bit {}, Remaining bits {}",
                    self.offset,
                    self.bits.len() - self.offset
                )
                .as_str(),
            ));
        }
        let bit = *self.bits.get(self.offset).as_deref().unwrap();
        Ok(bit)
    }

    fn get_bitvec(&mut self, length: usize) -> Result<BitVec<Msb0, u8>, AperCodecError> {
        if length + self.offset >= self.bits.len() {
            return Err(AperCodecError::new(
                format!(
                    "AperCodec:GetBitError:Requested Bit {}, Remaining bits {}",
                    self.offset,
                    self.bits.len() - self.offset
                )
                .as_str(),
            ));
        }
        let bv = BitVec::from_bitslice(&self.bits[self.offset..self.offset + length]);
        let _ = self.advance_maybe_err(length, true)?;

        Ok(bv)
    }
}
