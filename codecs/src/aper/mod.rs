#![allow(dead_code)]
//! ASN.1 Aligned PER Codec

pub mod error;

pub use error::Error as AperCodecError;

pub mod decode;

pub mod encode;

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
    decode_offset: usize,
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
            decode_offset: 0,
            key: None,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bits.into()
    }

    pub fn decode_align(&mut self) -> Result<(), AperCodecError> {
        if self.decode_offset % 8 == 0 {
            return Ok(());
        }

        let remaining = 8 - (self.decode_offset & 0x7_usize);
        log::trace!("Aligning Codec Buffer with {} bits", remaining);

        if !self.bits[self.decode_offset..self.decode_offset + remaining]
            .iter()
            .all(|b| b == false)
        {
            Err(AperCodecError::new(
                format!(
                    "{} Padding bits at Offset {} not all '0'.",
                    remaining, self.decode_offset,
                )
                .as_str(),
            ))
        } else {
            self.decode_offset += remaining;
            Ok(())
        }
    }

    fn decode_bool(&mut self) -> Result<bool, AperCodecError> {
        if self.bits.len() == self.decode_offset {
            return Err(AperCodecError::new(
                "AperCodec:DecodeError:End of Bitstream reached while trying to decode bool.",
            ));
        }
        let bit = *self.bits.get(self.decode_offset).as_deref().unwrap();
        let _ = self.advance_maybe_err(1, true)?;

        Ok(bit)
    }

    fn decode_bits_as_integer(&mut self, bits: usize) -> Result<i128, AperCodecError> {
        let remaining = self.bits.len() - self.decode_offset;
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
                self.decode_offset,
                bits
            );
            let value = if bits == 0 {
                0_i128
            } else {
                self.bits[self.decode_offset..self.decode_offset + bits].load_be::<u128>() as i128
            };
            log::trace!("Decoded Value: {:#?}", value);
            self.advance_maybe_err(bits, false)?;
            Ok(value)
        }
    }

    fn advance_maybe_err(&mut self, bits: usize, ignore: bool) -> Result<(), AperCodecError> {
        let offset = self.decode_offset + bits;
        if offset > self.bits.len() {
            if ignore {
                self.decode_offset = self.bits.len()
            } else {
                let remaining = self.bits.len() - self.decode_offset;
                return Err(AperCodecError::new(
                    format!(
                        "AperCodec:DecodeError:Requested Bits to advance {}, Remaining bits {}",
                        bits, remaining
                    )
                    .as_str(),
                ));
            }
        } else {
            self.decode_offset = offset
        }
        Ok(())
    }

    fn get_bit(&self) -> Result<bool, AperCodecError> {
        if self.decode_offset >= self.bits.len() {
            return Err(AperCodecError::new(
                format!(
                    "AperCodec:GetBitError:Requested Bit {}, Remaining bits {}",
                    self.decode_offset,
                    self.bits.len() - self.decode_offset
                )
                .as_str(),
            ));
        }
        let bit = *self.bits.get(self.decode_offset).as_deref().unwrap();
        Ok(bit)
    }

    fn get_bitvec(&mut self, length: usize) -> Result<BitVec<Msb0, u8>, AperCodecError> {
        if length + self.decode_offset >= self.bits.len() {
            return Err(AperCodecError::new(
                format!(
                    "AperCodec:GetBitError:Requested Bit {}, Remaining bits {}",
                    length,
                    self.bits.len() - self.decode_offset
                )
                .as_str(),
            ));
        }
        let bv = BitVec::from_bitslice(&self.bits[self.decode_offset..self.decode_offset + length]);
        let _ = self.advance_maybe_err(length, true)?;

        Ok(bv)
    }

    fn get_bytes(&mut self, length: usize) -> Result<Vec<u8>, AperCodecError> {
        let length = length * 8;
        if length + self.decode_offset >= self.bits.len() {
            return Err(AperCodecError::new(
                format!(
                    "AperCodec:GetBitError:Requested Bits {}, Remaining bits {}",
                    length,
                    self.bits.len() - self.decode_offset
                )
                .as_str(),
            ));
        }
        let mut bv = self.bits[self.decode_offset..self.decode_offset + length].to_bitvec();
        bv.force_align();
        self.advance_maybe_err(length, true)?;
        Ok(BitVec::into_vec(bv))
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
        log::trace!("AperCodecData: offset: {}", self.decode_offset);
    }

    // Encoding functions.

    /// Encode a bool.
    fn encode_bool(&mut self, value: bool) {
        self.bits.push(value);
    }

    /// Add bits to the encoding buffer.
    fn append_bits(&mut self, bits: &BitSlice<Msb0, u8>) {
        self.bits.extend_from_bitslice(bits);
    }

    /// Byte align the encoding buffer by padding with zero bits.
    fn align(&mut self) {
        let remaining = 8 - (self.bits.len() & 0x7_usize);
        if remaining < 8 {
            self.bits.resize(self.bits.len() + remaining, false);
        }
    }

    /// Get the length of the data in bytes
    /// This is useful when encoding an open type.
    pub fn length_in_bytes(&self) -> usize {
        ((self.bits.len() - 1) / 8) + 1
    }

    /// Append one encoding to another preserving byte alignment.
    /// This is useful when encoding an open type.
    pub fn append_aligned(&mut self, other: &mut Self) {
        self.align();
        other.align();
        self.append_bits(&other.bits)
    }
}

fn bytes_needed_for_range(range: i128) -> u8 {
    let bits_needed: u8 = 128 - range.leading_zeros() as u8;
    let mut bytes_needed = bits_needed / 8;
    if bits_needed % 8 != 0 {
        bytes_needed += 1
    }
    bytes_needed
}

#[cfg(test)]
mod tests {
    use super::*;

    // A test that would fail if it were not for the `force_align()` in AperCodecData::get_bytes().
    #[test]
    fn get_bytes_unaligned() {
        let mut d = AperCodecData::from_slice(&vec![0x0f, 0xf0]);
        let _ = d.get_bitvec(4);
        let bytes = d.get_bytes(1).unwrap();
        assert_eq!(bytes, vec![0xff]);
    }
}
