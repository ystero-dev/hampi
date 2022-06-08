//! Functionality for decoding character strings

use crate::aper::AperCodecData;
use crate::aper::AperCodecError;

use super::decode_internal::decode_length_determinent;

// 27.5.3 and 27.5.4
/// Decode a VisibleString CharacterString Type.
pub fn decode_visible_string(
    data: &mut AperCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<String, AperCodecError> {
    log::debug!(
        "decode_visible_string: lb: {:?}, ub: {:?}, is_extensible: {}",
        lb,
        ub,
        is_extensible
    );

    // Following values  are never used instead Canonical decode
    let (_val_lower, _val_higher) = (32u8, 127u8);

    let num_bits = 8; // N = 95, B = 7, B2 = 8

    let is_extended = if is_extensible {
        data.decode_bool()?
    } else {
        false
    };

    let length = if is_extended {
        decode_length_determinent(data, None, None, false)?
    } else {
        decode_length_determinent(data, lb, ub, false)?
    };

    let mut out = String::new();
    if length > 0 {
        let length = length * num_bits;

        if length > 16 {
            data.decode_align()?;
        }

        let bits = data.get_bitvec(length)?;
        let decoded = bits
            .chunks_exact(num_bits)
            .map(|c| c[0] as u8)
            .collect::<Vec<u8>>();
        let decoded = std::str::from_utf8(&decoded).unwrap();
        out += &decoded;
    }

    data.dump();

    Ok(out)
}

/// Decode a PrintableString CharacterString Type.
pub fn decode_printable_string(
    data: &mut AperCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<String, AperCodecError> {
    log::debug!(
        "decode_printable_string: lb: {:?}, ub: {:?}, is_extensible: {}",
        lb,
        ub,
        is_extensible
    );

    let (_val_lower, _val_higher) = (32u8, 122u8);

    let num_bits = 8; // N = 74, B = 7, B2 = 8

    let mut alphabet = vec![' ', '\'', '(', ')', '+', ',', '-', '.', '/'];
    alphabet.extend(('0'..='9').collect::<Vec<char>>());
    alphabet.extend(vec![':', '=', '?']);
    alphabet.extend(('a'..='z').collect::<Vec<char>>());
    alphabet.extend(('A'..='Z').collect::<Vec<char>>());

    let is_extended = if is_extensible {
        data.decode_bool()?
    } else {
        false
    };

    let length = if is_extended {
        decode_length_determinent(data, None, None, false)?
    } else {
        decode_length_determinent(data, lb, ub, false)?
    };

    let mut out = String::new();
    if length > 0 {
        let length = length * num_bits;

        if length > 16 {
            data.decode_align()?;
        }

        let bits = data.get_bitvec(length)?;
        let decoded = bits
            .chunks_exact(num_bits)
            .map(|c| c[0] as u8)
            .collect::<Vec<u8>>();
        let decoded = std::str::from_utf8(&decoded).unwrap();
        out += &decoded;
    }

    data.dump();

    Ok(out)
}

// UTF-8 String is always - indefinite length case as it's not a fixed character width string. It's
// almost like decoding an octet string.
// 27.6
/// Decode a UTF8String CharacterString Type.
pub fn decode_utf8_string(
    data: &mut AperCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<String, AperCodecError> {
    log::debug!(
        "decode_utf8_string: lb: {:?}, ub: {:?}, is_extensible: {}",
        lb,
        ub,
        is_extensible
    );

    let (_val_lower, _val_higher) = (0u8, 255u8);

    let num_bits = 8; // N = 74, B = 7, B2 = 8

    let length = decode_length_determinent(data, None, None, false)?;
    let mut out = String::new();
    if length > 0 {
        let length = length * num_bits;

        if length > 16 {
            data.decode_align()?;
        }

        let bits = data.get_bitvec(length)?;
        let decoded = bits
            .chunks_exact(num_bits)
            .map(|c| c[0] as u8)
            .collect::<Vec<u8>>();
        let decoded = String::from_utf8(decoded).unwrap();

        out += &decoded;
    }

    data.dump();

    Ok(out)
}
