#![allow(dead_code)]
//! ASN.1 Aligned PER Codec

pub mod error;

pub use error::Error as AperCodecError;

pub mod decode;

/// Trait representing an 'APER Codec'.
///
/// This 'trait' is to be derived by any `struct` or `enum` representing an ASN.1 Type.
pub trait AperCodec {
    type Output;

    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError>;

    fn encode(&self, _data: &mut AperCodecData) -> Result<(), AperCodecError> {
        todo!();
    }
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
    key: Option<i128>,
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
            key: None,
        }
    }

    fn decode_align(&mut self) -> Result<(), AperCodecError> {
        if self.offset % 8 == 0 {
            return Ok(());
        }

        let remaining = 8 - (self.offset & 0x7_usize);
        log::trace!("Aligning Codec Buffer with {} bits", remaining);

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

    fn align(&mut self) {
        let remaining = 8 - (self.offset & 0x7_usize);
        let mut bv = bitvec![Msb0, u8; 0; remaining];
        self.bits.append(&mut bv);
        self.offset += remaining;
    }

    fn decode_bool(&mut self) -> Result<bool, AperCodecError> {
        if self.bits.len() == self.offset {
            return Err(AperCodecError::new(
                "AperCodec:DecodeError:End of Bitstream reached while trying to decode bool.",
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
            log::trace!(
                "Decoding Bits as Integer. offset: {}, bits: {}",
                self.offset,
                bits
            );
            let value = if bits == 0 {
                0_i128
            } else {
                self.bits[self.offset..self.offset + bits].load_be::<u128>() as i128
            };
            log::trace!("Decoded Value: {:#?}", value);
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
                    length,
                    self.bits.len() - self.offset
                )
                .as_str(),
            ));
        }
        let bv = BitVec::from_bitslice(&self.bits[self.offset..self.offset + length]);
        let _ = self.advance_maybe_err(length, true)?;

        Ok(bv)
    }

    fn get_bytes(&mut self, length: usize) -> Result<Vec<u8>, AperCodecError> {
        let length = length * 8;
        if length + self.offset >= self.bits.len() {
            return Err(AperCodecError::new(
                format!(
                    "AperCodec:GetBitError:Requested Bits {}, Remaining bits {}",
                    length,
                    self.bits.len() - self.offset
                )
                .as_str(),
            ));
        }
        let bytes = BitVec::into_vec(self.bits[self.offset..self.offset + length].to_bitvec());
        let _ = self.advance_maybe_err(length, true)?;

        Ok(bytes)
    }

    /// Get's the current `key` value.
    ///
    /// This value will be used by a decoder to determine which 'decode' function is to be called
    /// (for example in an `enum`, it will be used to determine which `variant` of the `enum` will
    /// be decoded.
    pub fn get_key(&self) -> Option<i128> {
        self.key
    }

    /// Sets the current `key` value.
    ///
    /// During decoding 'open' types, the 'key' used to decode the type further is determined by a
    /// `key_value` field. ie. a field with attribute `key_value` set in a struct (derived from a
    /// SEQUENCE ASN.1 type.) This value is passed to the 'decoder' logic further through `set_key`
    /// function, which updates the internal state of the decoder data.
    pub fn set_key(&mut self, key: i128) {
        let _ = self.key.replace(key);
    }

    /// Dump current 'offset'.
    #[inline]
    pub fn dump(&self) {
        log::trace!("AperCodecData: offset: {}", self.offset);
    }
}
