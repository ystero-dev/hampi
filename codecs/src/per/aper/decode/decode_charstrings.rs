//! Functionality for decoding character strings

use crate::per::common::decode::decode_string_common;
use crate::per::PerCodecData;
use crate::PerCodecError;

// 27.5.3 and 27.5.4
/// Decode a VisibleString CharacterString Type.
pub fn decode_visible_string(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<String, PerCodecError> {
    log::debug!(
        "decode_visible_string: lb: {:?}, ub: {:?}, is_extensible: {}",
        lb,
        ub,
        is_extensible
    );
    decode_string_common(data, lb, ub, is_extensible, 8, true)
}

/// Decode a PrintableString CharacterString Type.
pub fn decode_printable_string(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<String, PerCodecError> {
    log::debug!(
        "decode_printable_string: lb: {:?}, ub: {:?}, is_extensible: {}",
        lb,
        ub,
        is_extensible
    );
    decode_string_common(data, lb, ub, is_extensible, 8, true)
}

// UTF-8 String is always - indefinite length case as it's not a fixed character width string. It's
// almost like decoding an octet string.
// 27.6
/// Decode a UTF8String CharacterString Type.
pub fn decode_utf8_string(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<String, PerCodecError> {
    log::debug!(
        "decode_utf8_string: lb: {:?}, ub: {:?}, is_extensible: {}",
        lb,
        ub,
        is_extensible
    );
    decode_string_common(data, lb, ub, is_extensible, 8, true)
}
