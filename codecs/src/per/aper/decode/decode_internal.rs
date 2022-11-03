//! Internal decode functions.
use std::convert::TryInto;

use crate::per::PerCodecData;

use super::AperCodecError;

// Decode a "Normally Small" non-negative number
//
// This is typically usedd when encoding/decoding Choice Indexes that are not present in the
// extension root.
pub(super) fn decode_normally_small_non_negative_whole_number(
    data: &mut PerCodecData,
) -> Result<i128, AperCodecError> {
    let is_small = data.decode_bool()?;
    if !is_small {
        data.decode_bits_as_integer(6, false)
    } else {
        decode_semi_constrained_whole_number(data, 0_i128)
    }
}
// Decode "Normally Small" Length Determinent
//
// This type of "length" determinent is used to encode bitmap length in the SEQUENCE extensions,
// TODO: Support for the case when the length is greater than 64. We almost never come across this
// case in practice, so right now it just Errors, if in real life we actually see this error for
// any time it might have to be implemented to take care of that case.
fn decode_normally_small_length_determinent(
    data: &mut PerCodecData,
) -> Result<usize, AperCodecError> {
    let is_small = data.decode_bool()?;
    if !is_small {
        Ok(data.decode_bits_as_integer(6, false)? as usize + 1_usize)
    } else {
        decode_indefinite_length_determinent(data)
    }
}

// Decode a Length Determinent (Section 10.9)
//
// Decodes a Length Determinent.
pub fn decode_length_determinent(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    normally_small: bool,
) -> Result<usize, AperCodecError> {
    // Normally small is told to us by caller and we don't care about `lb` and `ub` values in that
    // case. We simply follow the procedure as explained in 10.9.3.4
    log::debug!("decode_length_determinent:");

    if normally_small {
        return decode_normally_small_length_determinent(data);
    }

    let lb = if let Some(l) = lb {
        l.try_into().unwrap()
    } else {
        0usize
    };

    if let Some(ub) = ub {
        let ub: usize = ub.try_into().unwrap();
        if ub < 65_536 {
            if lb == ub {
                return Ok(ub);
            }

            // 10.9.3.3
            decode_constrained_length_determinent(data, lb, ub)
        } else {
            decode_indefinite_length_determinent(data)
        }
    } else {
        // ub is None, it's an unconstrained one
        decode_indefinite_length_determinent(data)
    }
}

// Called when `lb` and `ub` are known and the range is less than 64K
fn decode_constrained_length_determinent(
    data: &mut PerCodecData,
    lb: usize,
    ub: usize,
) -> Result<usize, AperCodecError> {
    log::trace!(
        "decode_constrained_length_determinent, lb: {}, ub: {}",
        lb,
        ub
    );

    let range = ub - lb + 1;

    if range <= 65536 {
        // Almost always for our use cases, so let's just use it.
        let length = decode_constrained_whole_number(data, lb as i128, ub as i128)?;
        log::trace!("decoded length : {}", length);

        data.dump();

        Ok(length as usize)
    } else {
        unimplemented!("Lengths larger than 65536 are not supported yet.")
    }
}

// Called when `ub` is not determined or `ub ` - `lb` is greater than 64K and in this case value of
// `lb` is don't care.
fn decode_indefinite_length_determinent(data: &mut PerCodecData) -> Result<usize, AperCodecError> {
    data.decode_align()?;
    let first = data.decode_bool()?;
    let length = if !first {
        data.decode_bits_as_integer(7, false)?
    } else {
        let second = data.decode_bool()?;
        if !second {
            data.decode_bits_as_integer(14, false)?
        } else {
            let length = data.decode_bits_as_integer(6, false)?;
            if !(1..=4).contains(&length) {
                return Err(AperCodecError::new("The value should be 1 to 4"));
            } else {
                length * 16384
            }
        }
    };

    data.dump();

    Ok(length.try_into().unwrap())
}

// Section 10.8 X.691
pub(super) fn decode_unconstrained_whole_number(
    data: &mut PerCodecData,
) -> Result<i128, AperCodecError> {
    log::trace!("decode_unconstrained_length:");

    let length = decode_length_determinent(data, None, None, false)?;
    let bits = length * 8;
    data.decode_bits_as_integer(bits, true)
}

// Section 10.7 X.691
pub(super) fn decode_semi_constrained_whole_number(
    data: &mut PerCodecData,
    lb: i128,
) -> Result<i128, AperCodecError> {
    log::trace!("decode_semi_constrained_whole_number:");

    let length = decode_length_determinent(data, None, None, false)?;

    let bits = length * 8;
    let val = data.decode_bits_as_integer(bits, false)?;

    Ok(val + lb)
}

// Decode a 'constrained' whole number where both `lb` and `ub` are available.
//
// From Section 10.5
pub(super) fn decode_constrained_whole_number(
    data: &mut PerCodecData,
    lb: i128,
    ub: i128,
) -> Result<i128, AperCodecError> {
    log::trace!("decode_constrained_whole_number: lb: {}, ub: {}", lb, ub,);

    let range = ub - lb + 1;
    if range <= 0 {
        Err(AperCodecError::new(
            "Range for the Integer Constraint is negative.",
        ))
    } else {
        let value = if range < 256 {
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
            data.decode_bits_as_integer(bits, false)?
        } else if range == 256 {
            data.decode_align()?;
            data.decode_bits_as_integer(8, false)?
        } else if range <= 65536 {
            data.decode_align()?;
            data.decode_bits_as_integer(16, false)?
        } else {
            let bytes_needed = crate::per::common::bytes_needed_for_range(range);
            log::trace!("bytes_needed : {}", bytes_needed);
            let length = decode_constrained_length_determinent(data, 1, bytes_needed as usize)?;
            data.decode_align()?;
            data.decode_bits_as_integer(length * 8, false)?
        };

        Ok(value + lb)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_decode_constrained_whole_number_range_0() {
        let data = &[0x70u8, 0, 0, 0];
        let mut codec_data = PerCodecData::from_slice(data);
        codec_data.advance_maybe_err(1, false).unwrap();
        let value = decode_constrained_whole_number(&mut codec_data, 14, 14);
        assert!(value.is_ok());
        let value = value.unwrap();
        assert_eq!(value, 14i128);
    }

    #[test]
    fn test_decode_constrained_whole_number_lt_256() {
        let data = &[0x70u8, 0, 0, 0];
        let mut codec_data = PerCodecData::from_slice(data);
        codec_data.advance_maybe_err(1, false).unwrap();
        let value = decode_constrained_whole_number(&mut codec_data, 7, 14);
        assert!(value.is_ok());
        let value = value.unwrap();
        assert_eq!(value, 14i128);
    }

    #[test]
    fn test_decode_constrained_whole_number_eq_256() {
        let data = &[0x80u8, 0x70u8, 0, 0];
        let mut codec_data = PerCodecData::from_slice(data);
        codec_data.advance_maybe_err(1, false).unwrap();
        let value = decode_constrained_whole_number(&mut codec_data, 0, 255);
        assert!(value.is_ok(), "{:#?}", value.err());
        let value = value.unwrap();
        assert_eq!(value, 0x70i128);
    }

    #[test]
    fn test_decode_constrained_whole_number_lt_64k() {
        let data = &[0x00u8, 0x70u8, 0x00, 1];
        let mut codec_data = PerCodecData::from_slice(data);
        codec_data.advance_maybe_err(12, false).unwrap();
        let value = decode_constrained_whole_number(&mut codec_data, 0, 64000);
        assert!(value.is_ok(), "{:#?}", value.err());
        let value = value.unwrap();
        assert_eq!(value, 1_i128);
    }

    #[test]
    fn test_decode_int_range_68719476735() {
        let mut data = PerCodecData::from_slice(&[0x00, 0x7B]);
        let value = decode_constrained_whole_number(&mut data, 0, 68719476735).unwrap();
        assert_eq!(value, 123);
    }

    #[test]
    fn test_decode_constrained_whole_number_gt_64k() {
        let data = &[0x00u8, 0x78u8, 0x01, 1, 0x01, 0x02];
        let mut codec_data = PerCodecData::from_slice(data);
        codec_data.advance_maybe_err(12, false).unwrap();

        // We are now looking at the 12th bit.
        // 0000 0000 0111 >> 1000 0000 0001 0000 0001 0000 0001 0000 0002

        let value = decode_constrained_whole_number(&mut codec_data, 0, 20_000_000);
        assert!(value.is_ok(), "{:#?}", value.err());
        let value = value.unwrap();

        // Bytes needed for range 0-20000000 = 4.
        // So the length of the value is in range 1-4, so get next 2 bits '10', so length = 3 bytes.
        // 0000 0000 0111 10 >> 00 0000 0001 0000 0001 0000 0001 0000 0002
        // Byte align.
        // 0000 0000 0111 1000 >> 0000 0001 0000 0001 0000 0001 0000 0002
        // Read the next 3 bytes to produce value 0x010101 = 65793.
        // 0000 0000 0111 1000 0000 0001 0000 0001 0000 0001 >> 0000 0002
        assert_eq!(value, 65793);

        // Prove we still have the last 8 bits to spare.
        codec_data.advance_maybe_err(8, false).unwrap();
    }
}
