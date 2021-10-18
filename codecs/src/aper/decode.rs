use crate::aper::AperCodecData;
use crate::aper::AperCodecError;

pub fn decode_choice_idx(lb: i128, ub: i128) -> Result<(), AperCodecError> {
    Ok(())
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

// Decode "Normally Small" Length Determinent
//
// This type of "length" determinent is used to encode bitmap length in the SEQUENCE extensions,
// TODO: Support for the case when the length is greater than 64. We almost never come across this
// case in practice, so right now it just Errors, if in real life we actually see this error for
// any time it might have to be implemented to take care of that case.
fn decode_normally_small_length_determinent(
    data: &mut AperCodecData,
) -> Result<usize, AperCodecError> {
    Ok(0 as usize)
}

fn decode_constrained_length_detereminent(
    data: &mut AperCodecData,
    lb: usize,
    ub: usize,
) -> Result<usize, AperCodecError> {
    Ok(0 as usize)
}

fn decode_unconstrained_length_determinent(
    data: &mut AperCodecData,
) -> Result<usize, AperCodecError> {
    Ok(0 as usize)
}

// Section 10.8 X.691
fn decode_unconstrained_whole_number(data: &mut AperCodecData) -> Result<i128, AperCodecError> {
    unimplemented!()
}

fn decode_semi_constrained_whole_number(
    data: &mut AperCodecData,
    lb: i128,
) -> Result<i128, AperCodecError> {
    unimplemented!("Not implemented Yet")
}

fn decode_constrained_whole_number(
    data: &mut AperCodecData,
    lb: i128,
    ub: i128,
) -> Result<i128, AperCodecError> {
    let range = ub - lb + 1;
    if range <= 0 {
        Err(AperCodecError::new(
            "Range for the Integer Constraint is negative.",
        ))
    } else {
        eprintln!("range: {}", range);
        if range < 256 {
            let bits = match range as u8 {
                0..=1 => 0,
                2 => 1,
                3..=4 => 2,
                5..=8 => 3,
                9..=16 => 4,
                17..=32 => 5,
                33..=64 => 6,
                65..=128 => 7,
                129..=255 => 8,
            };
            data.decode_bits_as_integer(bits)
        } else if range == 256 {
            let _ = data.decode_align()?;
            data.decode_bits_as_integer(8)
        } else if range < 65536 {
            let _ = data.decode_align()?;
            data.decode_bits_as_integer(16)
        } else {
            let bytes_needed = bytes_needed_for_range(range);
            let length = decode_constrained_length_detereminent(data, 1, bytes_needed as usize)?;
            let bits = (length + 1) * 8;
            let value = data.decode_bits_as_integer(bits);
            data.decode_bits_as_integer(bits)
        }
    }
}

fn bytes_needed_for_range(range: i128) -> u8 {
    match 128 - range.leading_zeros() {
        64..=128 => 16,
        32..=64 => 8,
        16..=32 => 4,
        8..=16 => 2,
        0..=8 => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_decode_constrained_whole_number_lt_256() {
        let data = &[0x70u8, 0, 0, 0];
        let mut codec_data = AperCodecData::from_slice(data);
        codec_data.advance_maybe_err(1, false).unwrap();
        let value = decode_constrained_whole_number(&mut codec_data, 7, 14);
        assert!(value.is_ok());
        let value = value.unwrap();
        assert_eq!(value, 7i128);
    }

    #[test]
    fn test_decode_constrained_whole_number_eq_256() {
        let data = &[0x80u8, 0x70u8, 0, 0];
        let mut codec_data = AperCodecData::from_slice(data);
        codec_data.advance_maybe_err(1, false).unwrap();
        let value = decode_constrained_whole_number(&mut codec_data, 0, 255);
        assert!(value.is_ok(), "{:#?}", value.err());
        let value = value.unwrap();
        assert_eq!(value, 0x70i128);
    }

    #[test]
    fn test_decode_constrained_whole_number_gt_256() {
        let data = &[0x00u8, 0x70u8, 0x00, 1];
        let mut codec_data = AperCodecData::from_slice(data);
        codec_data.advance_maybe_err(12, false).unwrap();
        let value = decode_constrained_whole_number(&mut codec_data, 0, 64000);
        assert!(value.is_ok(), "{:#?}", value.err());
        let value = value.unwrap();
        assert_eq!(value, 1_i128);
    }
}
