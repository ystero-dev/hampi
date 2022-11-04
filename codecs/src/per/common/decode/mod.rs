//! Decode APIs for APER Codec

mod decode_internal;

use bitvec::prelude::*;

use crate::{PerCodecData, PerCodecError};

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
    log::debug!(
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
            if length > 16 {
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
    log::debug!(
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
            if length > 2 {
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

    let num_bits = 8;
    let length = length * num_bits;
    if length > 16 {
        data.decode_align()?;
    }
    let bits = data.get_bitvec(length)?;
    let bytes = bits
        .chunks_exact(num_bits)
        .map(|c| c.load::<u8>())
        .collect::<Vec<u8>>();

    data.dump();

    std::str::from_utf8(&bytes)
        .map(|s| s.to_string())
        .map_err(|_| PerCodecError::new("UTF decode failed"))
}
