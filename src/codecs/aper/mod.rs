#![allow(dead_code)]
//! ASN.1 Aligned PER Codec

// FIXME: Move into a proper `serde` functionality, but let's build all the building blocks first.

#[derive(Debug)]
struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "aper_error!")
    }
}

impl std::error::Error for Error {}

use bitvec::prelude::*;

/// Structure representing an APER Codec.
///
/// While En(De)coding ASN.1 Types using the APER encoding scheme, the encoded data is stored in a
/// `BitVec`.
#[derive(Default)]
struct AperCodec {
    bits: BitVec<Msb0, u8>,
    offset: usize,
}

impl AperCodec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_i32(
        &mut self,
        extension: bool,
        _lb: Option<i32>,
        _ub: Option<i32>,
    ) -> Result<i32, Error> {
        // Extension bit is present
        if extension {
            let _extension_bit = self.bits.get(self.offset);
            self.offset += 1;
        }
        Ok(0_i32)
    }
}
