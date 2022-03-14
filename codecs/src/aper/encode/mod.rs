//! ASN.1 Aper Encoder module.

use crate::aper::AperCodecData;
use crate::aper::AperCodecError;

/// Encode a Choice Index
///
/// During Encoding a 'CHOICE' Type to help decoding, the 'CHOICE' Index is encoded first, followed
/// by the actual encoding of the 'CHOICE' variant.
pub fn encode_choice_idx(
    _data: &mut AperCodecData,
    _lb: i128,
    _ub: i128,
    _is_extensible: bool,
    _value: i128,
) -> Result<(), AperCodecError> {
    log::trace!("encode_bool");
    todo!();
}

/// Encode a BOOLEAN Value
///
/// Encodes a boolean value into the passed `AperCodecData` structure.
pub fn encode_bool(data: &mut AperCodecData, value: bool) -> Result<(), AperCodecError> {
    log::trace!("encode_bool");
    data.encode_bool(value)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn encode_bool_always_success() {
        let mut data = AperCodecData::new();

        let result = encode_bool(&mut data, true);
        assert!(result.is_ok());
        assert_eq!(data.offset, 1);
        assert_eq!(data.bits[0], true);
    }
}
