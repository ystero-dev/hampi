//! Decode APIs for APER Codec

mod decode_internal;

use bitvec::prelude::*;

use crate::per::PerCodecData;

use super::AperCodecError;

#[allow(unused)]
use decode_internal::*;

pub use decode_internal::decode_length_determinent;

/// Decode a Choice Index.
///
/// For an ASN.1 `CHOICE` Type, a CHOICE Index is first decoded. This function is used to `decode`
/// the choice index. Returns the Index in the 'root' or 'additions' and a flag indicated whether
/// the value is from the 'root_extensions' or 'addtions'. The caller would then decide the
/// appropriate `decode` function for the CHOICE variant is called.
pub fn decode_choice_idx(
    data: &mut PerCodecData,
    lb: i128,
    ub: i128,
    is_extensible: bool,
) -> Result<(i128, bool), AperCodecError> {
    log::debug!(
        "decode_choice_idx: lb: {}, ub: {}, extensible: {}",
        lb,
        ub,
        is_extensible
    );

    let (idx, extended) = if is_extensible {
        let extended = data.decode_bool()?;
        if !extended {
            let (idx, _) = decode_integer(data, Some(lb), Some(ub), false)?;
            (idx, extended)
        } else {
            let idx = decode_normally_small_non_negative_whole_number(data)?;
            (idx, extended)
        }
    } else {
        let (idx, _) = decode_integer(data, Some(lb), Some(ub), false)?;
        (idx, false)
    };

    data.dump();

    Ok((idx, extended))
}

/// Decode The Sequence Header
///
/// The Sequence Header consists of potentially two fields
/// 1. Whether `extensions` are present in the encoding
/// 2. Which of the OPTIONAL fields (if any) are present as a bitmap.
pub fn decode_sequence_header(
    data: &mut PerCodecData,
    is_extensible: bool,
    optional_count: usize,
) -> Result<(BitVec<u8, Msb0>, bool), AperCodecError> {
    log::debug!("decode_sequence_header: extensible: {}", is_extensible);

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

/// Decode an Integer
///
/// Given an Integer Specification with PER Visible Constraints, decode an Integer Value to obtain
/// the integer value which will always be returned as an i128 value.
///
/// Note: The maximum (and minimum) value to be decoded is limited to an `i128` value. For the
/// protocols that are currently supported this limit is acceptable.
///
/// `lb` and `ub` are upper and lower bounds as determined by the PER Constraints (and hence can be
/// `None` if no Constraints are not speicifed. `is_extensible` specifies whether the defined type
/// is extensible (as per PER Constraints). Returned value is the value of the Integer (i128) and
/// whether the value is outside the extension root (`bool`: `true` if value is outside the
/// extension root.).
pub fn decode_integer(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<(i128, bool), AperCodecError> {
    log::debug!(
        "decode_integer: Lower: {:?} Upper:{:?} Extensible: {}",
        lb,
        ub,
        is_extensible
    );

    let extended_value = if is_extensible {
        data.decode_bool()?
    } else {
        false
    };

    let value = if extended_value {
        // 12.1
        decode_unconstrained_whole_number(data)?
    } else {
        // 12.2
        match lb {
            None =>
            // 12.2.4
            {
                decode_unconstrained_whole_number(data)?
            }
            Some(lb) => {
                match ub {
                    None =>
                    // 12.2.3
                    {
                        decode_semi_constrained_whole_number(data, lb)?
                    }
                    Some(ub) => {
                        // 12.2.1 and 12.2.2
                        decode_constrained_whole_number(data, lb, ub)?
                    }
                }
            }
        }
    };

    data.dump();

    Ok((value, extended_value))
}

/// Decode a Boolean
///
/// Decode a Boolean value. Returns the decoded value as a `bool`.
pub fn decode_bool(data: &mut PerCodecData) -> Result<bool, AperCodecError> {
    log::debug!("decode_bool:");

    let result = data.decode_bool()?;

    data.dump();

    Ok(result)
}

/// Decode an Enumerated Value
///
/// Decodes an Enumerated value as an index into either `root_values` of the ENUMERATED or
/// `ext_values` of the ENUMERATED and also decodes a flag indicating where the value belongs. If
/// `false` the value is from the `root_values`, else the value is from the `ext_values` of the
/// ENUMERATED.
pub fn decode_enumerated(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<(i128, bool), AperCodecError> {
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
        let decoded = decode_integer(data, lb, ub, false)?;
        decoded.0
    } else {
        decode_normally_small_non_negative_whole_number(data)?
    };

    data.dump();

    Ok((decoded, is_extended))
}

/// Decode a Bit String
///
/// Decodes the value of the BIT STRING from the Buffer.
pub fn decode_bitstring(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<BitVec<u8, Msb0>, AperCodecError> {
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

    let mut bv = BitVec::new();
    loop {
        let length = if is_extended {
            decode_length_determinent(data, None, None, false)?
        } else {
            decode_length_determinent(data, lb, ub, false)?
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

/// Decode an OCTET STRING
///
/// Decodes the value of the OCTET STRING from the Buffer.
pub fn decode_octetstring(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<Vec<u8>, AperCodecError> {
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
            decode_length_determinent(data, None, None, false)?
        } else {
            decode_length_determinent(data, lb, ub, false)?
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

mod decode_charstrings;
pub use decode_charstrings::*;
