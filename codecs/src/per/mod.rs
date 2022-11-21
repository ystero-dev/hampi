#![allow(dead_code)]
//! ASN.1 PER Encoding

pub mod error;

pub mod aper;

pub mod uper;

mod common;

pub use error::Error as PerCodecError;

use bitvec::prelude::*;

/// Structure representing an APER Codec.
///
/// While En(De)coding ASN.1 Types using the APER encoding scheme, the encoded data is stored in a
/// `BitVec`.
#[derive(Default, Debug)]
pub struct PerCodecData {
    bits: BitVec<u8, Msb0>,
    decode_offset: usize,
    key: Option<i128>,
    aligned: bool,
}

impl PerCodecData {
    /// Default `PerCodecData` for AperCodec
    pub fn new_aper() -> Self {
        Self {
            aligned: true,
            ..Self::default()
        }
    }

    /// Default `PerCodecData` for UperCodec
    pub fn new_uper() -> Self {
        Self::default()
    }

    /// Create Our `PerCodecData` Structure from a slice of u8 for AperCodec
    pub fn from_slice_aper(bytes: &[u8]) -> Self {
        Self::from_slice_internal(bytes, true)
    }

    /// Create Our `PerCodecData` Structure from a slice of u8 for UperCodec
    pub fn from_slice_uper(bytes: &[u8]) -> Self {
        Self::from_slice_internal(bytes, false)
    }

    fn from_slice_internal(bytes: &[u8], aligned: bool) -> Self {
        Self {
            bits: BitSlice::<_, _>::from_slice(bytes).to_bitvec(),
            decode_offset: 0,
            key: None,
            aligned,
        }
    }

    /// Get's the inner buffer as a `Vec<u8>` consuming the struct.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bits.into()
    }

    /// Align to 8 bit boundry during decode.
    pub fn decode_align(&mut self) -> Result<(), PerCodecError> {
        if self.decode_offset % 8 == 0 {
            return Ok(());
        }

        let remaining = 8 - (self.decode_offset & 0x7_usize);
        log::debug!("Aligning Codec Buffer with {} bits", remaining);

        if !self.bits[self.decode_offset..self.decode_offset + remaining]
            .iter()
            .all(|b| b == false)
        {
            Err(PerCodecError::new(
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

    fn decode_bool(&mut self) -> Result<bool, PerCodecError> {
        if self.bits.len() == self.decode_offset {
            return Err(PerCodecError::new(
                "perCodec:DecodeError:End of Bitstream reached while trying to decode bool.",
            ));
        }
        let bit = *self.bits.get(self.decode_offset).as_deref().unwrap();
        self.advance_maybe_err(1, true)?;

        Ok(bit)
    }

    fn decode_bits_as_integer(&mut self, bits: usize, signed: bool) -> Result<i128, PerCodecError> {
        let remaining = self.bits.len() - self.decode_offset;
        if remaining < bits {
            Err(PerCodecError::new(
                format!(
                    "PerCodec:DecodeError:Requested Bits to decode {}, Remaining bits {}",
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
            let value = if !signed {
                if bits == 0 {
                    0_i128
                } else {
                    self.bits[self.decode_offset..self.decode_offset + bits].load_be::<u128>()
                        as i128
                }
            } else {
                match bits {
                    8 => {
                        let inner = self.bits[self.decode_offset..self.decode_offset + bits]
                            .load_be::<u128>() as i8;
                        inner as i128
                    }
                    16 => {
                        let inner = self.bits[self.decode_offset..self.decode_offset + bits]
                            .load_be::<u128>() as i16;
                        inner as i128
                    }
                    24 => {
                        let inner = self.bits[self.decode_offset..self.decode_offset + bits]
                            .load_be::<u32>() as u32;
                        let inner = if self.bits[self.decode_offset] {
                            // Bit is 1 negative no.
                            inner | 0xFF000000
                        } else {
                            inner & 0x00FFFFFF
                        };
                        let inner = inner as i32;
                        inner as i128
                    }
                    32 => {
                        let inner = self.bits[self.decode_offset..self.decode_offset + bits]
                            .load_be::<u128>() as i32;
                        inner as i128
                    }
                    40 => {
                        let inner = self.bits[self.decode_offset..self.decode_offset + bits]
                            .load_be::<u64>() as u64;
                        let inner = if self.bits[self.decode_offset] {
                            // Bit is 1 negative no.
                            inner | 0xFFFFFF0000000000
                        } else {
                            inner & 0x000000FFFFFFFFFF
                        };
                        let inner = inner as i64;
                        inner as i128
                    }
                    48 => {
                        let inner = self.bits[self.decode_offset..self.decode_offset + bits]
                            .load_be::<u64>() as u64;
                        let inner = if self.bits[self.decode_offset] {
                            // Bit is 1 negative no.
                            inner | 0xFFFF000000000000
                        } else {
                            inner & 0x0000FFFFFFFFFFFF
                        };
                        let inner = inner as i64;
                        inner as i128
                    }
                    56 => {
                        let inner = self.bits[self.decode_offset..self.decode_offset + bits]
                            .load_be::<u64>() as u64;
                        let inner = if self.bits[self.decode_offset] {
                            // Bit is 1 negative no.
                            inner | 0xFF00000000000000
                        } else {
                            inner & 0x00FFFFFFFFFFFFFF
                        };
                        let inner = inner as i64;
                        inner as i128
                    }
                    64 => {
                        let inner = self.bits[self.decode_offset..self.decode_offset + bits]
                            .load_be::<u128>() as i64;
                        inner as i128
                    }
                    128 => {
                        let inner = self.bits[self.decode_offset..self.decode_offset + bits]
                            .load_be::<u128>() as i128;
                        inner as i128
                    }
                    _ => {
                        return Err(
                            PerCodecError::new(
                                format!(
                                    "For a signed number in 2's compliment form, requested bits {} not supported!",
                                    bits)));
                    }
                }
            };
            log::trace!("Decoded Value: {:?}", value);
            self.advance_maybe_err(bits, false)?;
            Ok(value)
        }
    }

    fn advance_maybe_err(&mut self, bits: usize, ignore: bool) -> Result<(), PerCodecError> {
        let offset = self.decode_offset + bits;
        if offset > self.bits.len() {
            if ignore {
                self.decode_offset = self.bits.len()
            } else {
                let remaining = self.bits.len() - self.decode_offset;
                return Err(PerCodecError::new(
                    format!(
                        "PerCodec:DecodeError:Requested Bits to advance {}, Remaining bits {}",
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

    fn get_bit(&self) -> Result<bool, PerCodecError> {
        if self.decode_offset >= self.bits.len() {
            return Err(PerCodecError::new(
                format!(
                    "PerCodec:GetBitError:Requested Bit {}, Remaining bits {}",
                    self.decode_offset,
                    self.bits.len() - self.decode_offset
                )
                .as_str(),
            ));
        }
        let bit = *self.bits.get(self.decode_offset).as_deref().unwrap();
        Ok(bit)
    }

    fn get_bitvec(&mut self, length: usize) -> Result<BitVec<u8, Msb0>, PerCodecError> {
        if length + self.decode_offset > self.bits.len() {
            return Err(PerCodecError::new(
                format!(
                    "PerCodec:GetBitError:Requested Bit {}, Remaining bits {}",
                    length,
                    self.bits.len() - self.decode_offset
                )
                .as_str(),
            ));
        }
        let bv = BitVec::from_bitslice(&self.bits[self.decode_offset..self.decode_offset + length]);
        self.advance_maybe_err(length, true)?;

        Ok(bv)
    }

    fn get_bytes(&mut self, length: usize) -> Result<Vec<u8>, PerCodecError> {
        let length = length * 8;
        if length + self.decode_offset > self.bits.len() {
            return Err(PerCodecError::new(
                format!(
                    "PerCodec:GetBitError:Requested Bits {}, Remaining bits {}",
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

    pub fn get_inner(&self) -> Result<Vec<u8>, PerCodecError> {
        Ok(BitVec::into_vec(self.bits.to_bitvec()))
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
        log::debug!("PerCodecData: offset: {}", self.decode_offset);
    }

    #[inline]
    pub fn dump_encode(&self) {
        log::debug!("PerCodecData: current_len : {}", self.bits.len());
    }

    /// Reserve certain bits at the current `offset`.
    #[inline]
    pub fn reserve(&mut self, count: usize) {
        self.bits.reserve(count);
        self.decode_offset = count;
    }

    /// `seek` pointer to the offset in the internal buffer
    #[inline]
    pub fn seek(&mut self, offset: usize) {
        self.decode_offset = offset;
    }

    pub fn swap_bits(&mut self, other: &mut BitSlice<u8, Msb0>, offset: usize) {
        self.bits[offset..other.len() + offset].swap_with_bitslice(other);
    }

    pub fn set_bit(&mut self, index: usize, value: bool) {
        self.bits.set(index, value);
    }
    // Encoding functions.

    /// Encode a bool.
    fn encode_bool(&mut self, value: bool) {
        self.bits.push(value);
    }

    /// Add bits to the encoding buffer.
    fn append_bits(&mut self, bits: &BitSlice<u8, Msb0>) {
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
