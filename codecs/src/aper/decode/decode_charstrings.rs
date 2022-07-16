//! Functionality for decoding character strings
use bitvec::field::BitField;

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
    decode_string(data, lb, ub, is_extensible)
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
    decode_string(data, lb, ub, is_extensible)
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
    decode_string(data, lb, ub, is_extensible)
}

fn decode_string(
    data: &mut AperCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<String, AperCodecError> {
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
        .map_err(|_| AperCodecError::new("UTF decode failed"))
}
