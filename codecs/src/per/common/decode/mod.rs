//! Decode APIs for APER Codec

mod decode_internal;

use bitvec::prelude::*;

use crate::{PerCodecData, PerCodecError, PerCodecErrorCause};

use std::convert::TryFrom;

#[allow(unused)]
use decode_internal::*;

pub(crate) use decode_internal::decode_length_determinent_common;

// Common decode functions used by the API functions of the codec. The API functions call the
// common functions For example, `decode_choice_idx` API will call `decode_choice_idx_common` fro
// APER Codec by passing aligned as `true`.

// Common decode function for choice index
pub fn decode_choice_idx_common(
    data: &mut PerCodecData,
    lb: i128,
    ub: i128,
    is_extensible: bool,
    aligned: bool,
) -> Result<(i128, bool), PerCodecError> {
    let (idx, extended) = if is_extensible {
        let extended = data.decode_bool()?;
        if !extended {
            let (idx, _) = decode_integer_common(data, Some(lb), Some(ub), false, aligned)?;
            (idx, extended)
        } else {
            let idx = decode_normally_small_non_negative_whole_number_common(data, aligned)?;
            (idx, extended)
        }
    } else {
        let (idx, _) = decode_integer_common(data, Some(lb), Some(ub), false, aligned)?;
        (idx, false)
    };

    data.dump();

    Ok((idx, extended))
}

// Common decode function for sequence header.
pub fn decode_sequence_header_common(
    data: &mut PerCodecData,
    is_extensible: bool,
    optional_count: usize,
    _aligned: bool,
) -> Result<(BitVec<u8, Msb0>, bool), PerCodecError> {
    let extended = if is_extensible {
        data.decode_bool()?
    } else {
        false
    };

    let mut bitmap = BitVec::new();
    if optional_count > 0 {
        log::trace!("{:?} optionals found", optional_count);
        bitmap.extend(data.get_bitvec(optional_count)?);
    }
    data.dump();
    Ok((bitmap, extended))
}

// Common function to decode INTEGER.
pub fn decode_integer_common(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
    aligned: bool,
) -> Result<(i128, bool), PerCodecError> {
    let extended_value = if is_extensible {
        data.decode_bool()?
    } else {
        false
    };

    let value = if extended_value {
        // 12.1
        decode_unconstrained_whole_number_common(data, aligned)?
    } else {
        // 12.2
        match lb {
            None =>
            // 12.2.4
            {
                decode_unconstrained_whole_number_common(data, aligned)?
            }
            Some(lb) => {
                match ub {
                    None =>
                    // 12.2.3
                    {
                        decode_semi_constrained_whole_number_common(data, lb, aligned)?
                    }
                    Some(ub) => {
                        // 12.2.1 and 12.2.2
                        decode_constrained_whole_number_common(data, lb, ub, aligned)?
                    }
                }
            }
        }
    };

    data.dump();

    Ok((value, extended_value))
}

// Common function to decode a REAL value
pub(crate) fn decode_real_common(
    data: &mut PerCodecData,
    aligned: bool,
) -> Result<f64, PerCodecError> {
    // ITU X.691 section 15 notes that this value starts with a length
    // determinant. For now, UPER and APER are identical other than
    // determining alignment for the length determinant.
    let length = decode_length_determinent_common(data, None, None, false, aligned)?;
    let decoded_value: f64;

    // ITU X.691 section 15 largely refers to ITU X.690 section 8.5 for REAL
    // encoding and decoding.
    if length == 0 {
        // ITU X.690 section 8.5.2: if there is no content, then return +0.
        decoded_value = 0.;
    } else {
        let first_byte = data.decode_bits_as_integer(8, false)?;
        if let Ok(first_byte) = u8::try_from(first_byte) {
            // ITU X.690 section 8.5.6: determine whether this is a special
            // value. If not, figure out which base this is encoded with.
            // Section 8.5.9 covers special values, section 8.5.7 covers
            // binary encoding of base 2/8/16, and section 8.5.8 covers
            // ISO 6093 encoding of base 10 values.
            match first_byte {
                super::NEGATIVE_ZERO => {
                    decoded_value = -0.;
                }
                super::INFINITY => {
                    decoded_value = f64::INFINITY;
                }
                super::NEGATIVE_INFINITY => {
                    decoded_value = f64::NEG_INFINITY;
                }
                super::NOT_A_NUMBER => {
                    decoded_value = f64::NAN;
                }
                super::BASE_10_NR1 | super::BASE_10_NR2 | super::BASE_10_NR3 => {
                    // Subtract 1 from length, since first byte was metadata.
                    // We currently decode ISO 6093 NR1, NR2, and NR3 all using
                    // the same underlying approach.
                    log::trace!(
                        "Decoding REAL as a base 10 value using ISO 6093 with {} bytes",
                        length - 1
                    );
                    decoded_value = decode_real_as_decimal(data, length - 1)?;
                }
                first_byte => {
                    if real_uses_binary(first_byte) {
                        // Subtract 1 from length, since first byte was metadata.
                        // We currently decode base 2, base 8, and base 16 all
                        // using the same underlying approach.
                        log::trace!("Decoding REAL as a binary value with {} bytes", length - 1);
                        decoded_value = decode_real_as_binary(first_byte, data, length - 1)?;
                    } else {
                        // TODO: Change cause to InvalidValue once it is supported
                        return Err(PerCodecError::new(
                            PerCodecErrorCause::Generic,
                            format!(
                                "Can only decode REAL values with reserved values, binary-encoded values, or base 10 values (encoded first byte: {:b})",
                                first_byte
                            )
                        ));
                    }
                }
            }
        } else {
            return Err(PerCodecError::new(
                PerCodecErrorCause::Generic,
                "Could not convert i128 with 8 data bits into a u8; please contact the developers",
            ));
        }
    }

    data.dump();
    Ok(decoded_value)
}

// Common function to decode a Boolean
pub fn decode_bool_common(data: &mut PerCodecData, _aligned: bool) -> Result<bool, PerCodecError> {
    let result = data.decode_bool()?;

    data.dump();

    Ok(result)
}

// Common function to decode an Enumerated Value
pub fn decode_enumerated_common(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
    aligned: bool,
) -> Result<(i128, bool), PerCodecError> {
    log::trace!(
        "decode_enumerated: lb: {:?}, ub: {:?}, is_extensible: {}",
        lb,
        ub,
        is_extensible
    );

    let is_extended = if is_extensible {
        data.decode_bool()?
    } else {
        false
    };

    let decoded = if !is_extended {
        let decoded = decode_integer_common(data, lb, ub, false, aligned)?;
        decoded.0
    } else {
        decode_normally_small_non_negative_whole_number_common(data, aligned)?
    };

    data.dump();

    Ok((decoded, is_extended))
}

// Common function to decode a Bit String
pub fn decode_bitstring_common(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
    aligned: bool,
) -> Result<BitVec<u8, Msb0>, PerCodecError> {
    let is_extended = if is_extensible {
        data.decode_bool()?
    } else {
        false
    };

    let mut bv = BitVec::new();
    loop {
        let length = if is_extended {
            decode_length_determinent_common(data, None, None, false, aligned)?
        } else {
            decode_length_determinent_common(data, lb, ub, false, aligned)?
        };

        if length > 0 {
            if length > 16 && aligned {
                data.decode_align()?;
            }
            bv.extend(data.get_bitvec(length)?);
        }

        // Fragmented So get the chunks in multiples of 16384,
        if length >= 16384 {
            continue;
        } else {
            break;
        }
    }

    data.dump();

    Ok(bv)
}

// Common function to decode an OCTET STRING
pub fn decode_octetstring_common(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
    aligned: bool,
) -> Result<Vec<u8>, PerCodecError> {
    log::trace!(
        "decode_bitstring: lb: {:?}, ub: {:?}, is_extensible: {}",
        lb,
        ub,
        is_extensible
    );

    let is_extended = if is_extensible {
        data.decode_bool()?
    } else {
        false
    };

    let mut octets = Vec::new();
    loop {
        let length = if is_extended {
            decode_length_determinent_common(data, None, None, false, aligned)?
        } else {
            decode_length_determinent_common(data, lb, ub, false, aligned)?
        };

        if length > 0 {
            if length > 2 && aligned {
                data.decode_align()?;
            }
            octets.extend(data.get_bytes(length)?);
        }

        // Fragmented So get the chunks in multiples of 16384,
        if length >= 16384 {
            continue;
        } else {
            break;
        }
    }

    data.dump();

    Ok(octets)
}

pub(crate) fn decode_string_common(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
    bits_per_char: usize,
    aligned: bool,
) -> Result<String, PerCodecError> {
    let is_extended = if is_extensible {
        data.decode_bool()?
    } else {
        false
    };

    let length = if is_extended {
        decode_length_determinent_common(data, None, None, false, aligned)?
    } else {
        decode_length_determinent_common(data, lb, ub, false, aligned)?
    };

    let length = length * bits_per_char;
    if length > 16 && aligned {
        data.decode_align()?;
    }
    let bits = data.get_bitvec(length)?;
    let bytes = bits
        .chunks_exact(bits_per_char)
        .map(|c| {
            let mut v = c.to_bitvec();
            let howmany = 8 - bits_per_char;
            for _ in 0..howmany {
                v.insert(0, false);
            }
            v.load_be::<u8>()
        })
        .collect::<Vec<u8>>();

    data.dump();

    std::str::from_utf8(&bytes)
        .map(|s| s.to_string())
        .map_err(|_| PerCodecError::new(PerCodecErrorCause::Generic, "UTF decode failed"))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_decode_real_base_2() {
        let data = &[0x03, 0x80, 0xFB, 0x05];
        let mut codec_data = PerCodecData::from_slice_aper(data);
        let value = decode_real_common(&mut codec_data, true);
        assert!(value.is_ok(), "{:#?}", value.err().unwrap());
        let value = value.unwrap();
        assert_eq!(value, 0.15625);
    }

    #[test]
    fn test_decode_real_base_8() {
        let data = &[0x03, 0x90, 0xFE, 0x0A];
        let mut codec_data = PerCodecData::from_slice_aper(data);
        let value = decode_real_common(&mut codec_data, true);
        assert!(value.is_ok(), "{:#?}", value.err().unwrap());
        let value = value.unwrap();
        assert_eq!(value, 0.15625);
    }

    #[test]
    fn test_decode_real_base_16() {
        let data = &[0x03, 0xAC, 0xFE, 0x05];
        let mut codec_data = PerCodecData::from_slice_aper(data);
        let value = decode_real_common(&mut codec_data, true);
        assert!(value.is_ok(), "{:#?}", value.err().unwrap());
        let value = value.unwrap();
        assert_eq!(value, 0.15625);
    }

    #[test]
    fn test_decode_real_base_10_nr1() {
        let data = &[0x04, super::super::BASE_10_NR1, b'1', b'2', b'3'];
        let mut codec_data = PerCodecData::from_slice_aper(data);
        let value = decode_real_common(&mut codec_data, true);
        assert!(value.is_ok(), "{:#?}", value.err().unwrap());
        let value = value.unwrap();
        assert_eq!(value, 123.0f64);
    }

    #[test]
    fn test_decode_real_base_10_nr2() {
        let data = &[
            0x08,
            super::super::BASE_10_NR2,
            b'0',
            b'.',
            b'1',
            b'5',
            b'6',
            b'2',
            b'5',
        ];
        let mut codec_data = PerCodecData::from_slice_aper(data);
        let value = decode_real_common(&mut codec_data, true);
        assert!(value.is_ok(), "{:#?}", value.err().unwrap());
        let value = value.unwrap();
        assert_eq!(value, 0.15625);
    }

    #[test]
    fn test_decode_real_base_10_nr3() {
        let data = &[
            0x0A,
            super::super::BASE_10_NR2,
            b'1',
            b'.',
            b'5',
            b'6',
            b'2',
            b'5',
            b'e',
            b'-',
            b'1',
        ];
        let mut codec_data = PerCodecData::from_slice_aper(data);
        let value = decode_real_common(&mut codec_data, true);
        assert!(value.is_ok(), "{:#?}", value.err().unwrap());
        let value = value.unwrap();
        assert_eq!(value, 0.15625);
    }
}
