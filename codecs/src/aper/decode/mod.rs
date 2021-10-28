use crate::aper::AperCodecData;
use crate::aper::AperCodecError;

mod decode_internal;
use decode_internal::*;

/// Decode a Choice Index.
///
/// For an ASN.1 `CHOICE` Type, a CHOICE Index is first decoded. This function is used to `decode`
/// the choice index. Returns the Index in the 'root' or 'additions' and a flag indicated whether
/// the value is from the 'root_extensions' or 'addtions'. The caller would then decide the
/// appropriate `decode` function for the CHOICE variant is called.
pub fn decode_choice_idx(
    data: &mut AperCodecData,
    lb: i128,
    ub: i128,
    is_extensible: bool,
) -> Result<(i128, bool), AperCodecError> {
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
    Ok((idx, extended))
}

/// Decode an Integer
///
/// Given an Integer Specification with PER Visible Constraints, decode an Integer Value to obtain
/// the integer value which will always be returned as an i128 value.
///
/// `lb` and `ub` are upper and lower bounds as determined by the PER Constraints (and hence can be
/// `None` if no Constraints are not speicifed. `is_extensible` specifies whether the defined type
/// is extensible (as per PER Constraints). Returned value is the value of the Integer (i128) and
/// whether the value is outside the extension root (`bool`: `true` if value is outside the
/// extension root.).
pub fn decode_integer(
    data: &mut AperCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<(i128, bool), AperCodecError> {
    let extended_value = if is_extensible {
        let v = data.decode_bool()?;
        v
    } else {
        false
    };

    let value = if extended_value {
        // 12.1
        decode_unconstrained_whole_number(data)?
    } else {
        // 12.2
        if lb.is_none() {
            // 12.2.4
            decode_unconstrained_whole_number(data)?
        } else {
            let lb = lb.unwrap();
            if ub.is_none() {
                // 12.2.3
                decode_semi_constrained_whole_number(data, lb)?
            } else {
                let ub = ub.unwrap();
                // 12.2.1 and 12.2.2
                decode_constrained_whole_number(data, lb, ub)?
            }
        }
    };
    Ok((value, extended_value))
}

/// Decode a Boolean
///
/// Decode a Boolean value. Returns the decoded value as a `bool`.
pub fn decode_bool(data: &mut AperCodecData) -> Result<bool, AperCodecError> {
    data.decode_bool()
}

/// Decode an Enumerated Value
///
/// Decodes an Enumerated value as an index into either `root_values` of the ENUMERATED or
/// `ext_values` of the ENUMERATED and also decodes a flag indicating where the value velongs. If
/// `false` the value is from the `root_values`, else the value is from the `ext_values` of the
/// ENUMERATED.
pub fn decode_enumerated(
    data: &mut AperCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    is_extensible: bool,
) -> Result<(i128, bool), AperCodecError> {
    let is_extended = if is_extensible {
        let is_extended = data.decode_bool()?;
        is_extended
    } else {
        false
    };

    let decoded = if !is_extended {
        let decoded = decode_integer(data, lb, ub, false)?;
        decoded.0
    } else {
        decode_normally_small_non_negative_whole_number(data)?
    };

    Ok((decoded, is_extended))
}

// Decode a "Normally Small" non-negative number
//
// This is typically usedd when encoding/decoding Choice Indexes that are not present in the
// extension root.
fn decode_normally_small_non_negative_whole_number(
    data: &mut AperCodecData,
) -> Result<i128, AperCodecError> {
    let is_small = data.decode_bool()?;
    if !is_small {
        data.decode_bits_as_integer(6)
    } else {
        decode_semi_constrained_whole_number(data, 0_i128)
    }
}
