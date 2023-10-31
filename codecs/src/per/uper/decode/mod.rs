//! Decode APIs for UPER Codec

use bitvec::prelude::*;

#[allow(unused)]
use crate::per::common::decode::*;
use crate::{PerCodecData, PerCodecError};

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
) -> Result<(i128, bool), PerCodecError> {
    log::trace!(
        "decode_choice_idx: lb: {}, ub: {}, extensible: {}",
        lb,
        ub,
        is_extensible
    );

    decode_choice_idx_common(data, lb, ub, is_extensible, false)
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
) -> Result<(BitVec<u8, Msb0>, bool), PerCodecError> {
    log::trace!("decode_sequence_header: extensible: {}", is_extensible);

    decode_sequence_header_common(data, is_extensible, optional_count, false)
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
) -> Result<(i128, bool), PerCodecError> {
    log::trace!(
        "decode_integer: Lower: {:?} Upper:{:?} Extensible: {}",
        lb,
        ub,
        is_extensible
    );

    decode_integer_common(data, lb, ub, is_extensible, false)
}

/// Decode a Boolean
///
/// Decode a Boolean value. Returns the decoded value as a `bool`.
pub fn decode_bool(data: &mut PerCodecData) -> Result<bool, PerCodecError> {
    log::trace!("decode_bool:");

    decode_bool_common(data, false)
}

/// Decode a Real
///
/// Decode a Real value. Returns the decoded value as a `f64`.
pub fn decode_real(data: &mut PerCodecData) -> Result<f64, PerCodecError> {
    log::trace!("decode_real:");
    decode_real_common(data, false)
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
) -> Result<(i128, bool), PerCodecError> {
    log::trace!(
        "decode_enumerated: lb: {:?}, ub: {:?}, is_extensible: {}",
        lb,
        ub,
        is_extensible
    );

    decode_enumerated_common(data, lb, ub, is_extensible, false)
}

/// Decode a Bit String
///
/// Decodes the value of the BIT STRING from the Buffer.
pub fn decode_bitstring(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<BitVec<u8, Msb0>, PerCodecError> {
    log::trace!(
        "decode_bitstring: lb: {:?}, ub: {:?}, is_extensible: {}",
        lb,
        ub,
        is_extensible
    );

    decode_bitstring_common(data, lb, ub, is_extensible, false)
}

/// Decode an OCTET STRING
///
/// Decodes the value of the OCTET STRING from the Buffer.
pub fn decode_octetstring(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<Vec<u8>, PerCodecError> {
    log::trace!(
        "decode_bitstring: lb: {:?}, ub: {:?}, is_extensible: {}",
        lb,
        ub,
        is_extensible
    );

    decode_octetstring_common(data, lb, ub, is_extensible, false)
}

/// Decodes a Length determinent
pub fn decode_length_determinent(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    normally_small: bool,
) -> Result<usize, PerCodecError> {
    log::trace!("decode_length_determinent:");

    decode_length_determinent_common(data, lb, ub, normally_small, false)
}

mod decode_charstrings;
pub use decode_charstrings::*;
