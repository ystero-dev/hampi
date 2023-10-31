//! Internal decode functions.
use std::convert::{TryInto, TryFrom};

use crate::{PerCodecData, PerCodecError, PerCodecErrorCause};

// Decode a "Normally Small" non-negative number
//
// This is typically usedd when encoding/decoding Choice Indexes that are not present in the
// extension root.
pub(super) fn decode_normally_small_non_negative_whole_number_common(
    data: &mut PerCodecData,
    aligned: bool,
) -> Result<i128, PerCodecError> {
    let is_small = data.decode_bool()?;
    if !is_small {
        data.decode_bits_as_integer(6, false)
    } else {
        decode_semi_constrained_whole_number_common(data, 0_i128, aligned)
    }
}
// Decode "Normally Small" Length Determinent
//
// This type of "length" determinent is used to encode bitmap length in the SEQUENCE extensions,
// TODO: Support for the case when the length is greater than 64. We almost never come across this
// case in practice, so right now it just Errors, if in real life we actually see this error for
// any time it might have to be implemented to take care of that case.
fn decode_normally_small_length_determinent_common(
    data: &mut PerCodecData,
    aligned: bool,
) -> Result<usize, PerCodecError> {
    let is_small = data.decode_bool()?;
    if !is_small {
        Ok(data.decode_bits_as_integer(6, false)? as usize + 1_usize)
    } else {
        decode_indefinite_length_determinent_common(data, aligned)
    }
}

// Decode a Length Determinent (Section 10.9)
//
// Decodes a Length Determinent.
pub(crate) fn decode_length_determinent_common(
    data: &mut PerCodecData,
    lb: Option<i128>,
    ub: Option<i128>,
    normally_small: bool,
    aligned: bool,
) -> Result<usize, PerCodecError> {
    // Normally small is told to us by caller and we don't care about `lb` and `ub` values in that
    // case. We simply follow the procedure as explained in 10.9.3.4
    if normally_small {
        return decode_normally_small_length_determinent_common(data, aligned);
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
            decode_constrained_length_determinent_common(data, lb, ub, aligned)
        } else {
            decode_indefinite_length_determinent_common(data, aligned)
        }
    } else {
        // ub is None, it's an unconstrained one
        decode_indefinite_length_determinent_common(data, aligned)
    }
}

// Called when `lb` and `ub` are known and the range is less than 64K
fn decode_constrained_length_determinent_common(
    data: &mut PerCodecData,
    lb: usize,
    ub: usize,
    aligned: bool,
) -> Result<usize, PerCodecError> {
    log::trace!(
        "decode_constrained_length_determinent, lb: {}, ub: {}",
        lb,
        ub
    );

    let range = ub - lb + 1;

    if range <= 65536 {
        // Almost always for our use cases, so let's just use it.
        let length = decode_constrained_whole_number_common(data, lb as i128, ub as i128, aligned)?;
        log::trace!("decoded length : {}", length);

        data.dump();

        Ok(length as usize)
    } else {
        unimplemented!("Lengths larger than 65536 are not supported yet.")
    }
}

// Called when `ub` is not determined or `ub ` - `lb` is greater than 64K and in this case value of
// `lb` is don't care.
fn decode_indefinite_length_determinent_common(
    data: &mut PerCodecData,
    aligned: bool,
) -> Result<usize, PerCodecError> {
    if aligned {
        data.decode_align()?;
    }
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
                return Err(PerCodecError::new(
                    PerCodecErrorCause::Generic,
                    "The value should be 1 to 4",
                ));
            } else {
                length * 16384
            }
        }
    };

    data.dump();

    Ok(length.try_into().unwrap())
}

// Section 10.8 X.691
pub(super) fn decode_unconstrained_whole_number_common(
    data: &mut PerCodecData,
    aligned: bool,
) -> Result<i128, PerCodecError> {
    log::trace!("decode_unconstrained_length:");

    let length = decode_length_determinent_common(data, None, None, false, aligned)?;
    let bits = length * 8;
    data.decode_bits_as_integer(bits, true)
}

// Section 10.7 X.691
pub(super) fn decode_semi_constrained_whole_number_common(
    data: &mut PerCodecData,
    lb: i128,
    aligned: bool,
) -> Result<i128, PerCodecError> {
    log::trace!("decode_semi_constrained_whole_number:");

    let length = decode_length_determinent_common(data, None, None, false, aligned)?;

    let bits = length * 8;
    let val = data.decode_bits_as_integer(bits, false)?;

    Ok(val + lb)
}

// Decode a 'constrained' whole number where both `lb` and `ub` are available.
//
// Refer to Section 10.5.6 when `aligned` is `false` and 10.5.7 when `aligned` is `true`
pub(super) fn decode_constrained_whole_number_common(
    data: &mut PerCodecData,
    lb: i128,
    ub: i128,
    aligned: bool,
) -> Result<i128, PerCodecError> {
    log::trace!("decode_constrained_whole_number: lb: {}, ub: {}", lb, ub,);

    let range = ub - lb + 1;
    if range <= 0 {
        Err(PerCodecError::new(
            PerCodecErrorCause::Generic,
            "Range for the Integer Constraint is negative.",
        ))
    } else {
        if aligned {
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
                let length = decode_constrained_length_determinent_common(
                    data,
                    1,
                    bytes_needed as usize,
                    aligned,
                )?;
                data.decode_align()?;
                data.decode_bits_as_integer(length * 8, false)?
            };

            Ok(value + lb)
        } else {
            if range > 1 {
                let leading_zeros = (range - 1).leading_zeros();
                let bits = 128 - leading_zeros as usize;
                let value = data.decode_bits_as_integer(bits, false)?;
                Ok(value + lb)
            } else {
                Ok(lb)
            }
        }
    }
}

pub(super) fn real_uses_binary(first_byte: u8) -> bool {
    first_byte & 0b1000_0000 == 0b1000_0000
}

// ITU X.690 section 8.5.7.1: get the value of the sign
fn get_sign(first_byte: u8) -> i8 {
    if first_byte & 0b0100_0000 == 0b0100_0000 { -1 } else { 1 }
}

// ITU X.690 section 8.5.7.2: get the value of the base used for encoding
fn get_base(first_byte: u8) -> Result<u8, PerCodecError> {
    // Get the log_2(2) value
    match first_byte & 0b0011_0000 {
        0b0000_0000 => {
            Ok(2)
        }
        0b0001_0000 => {
            Ok(8)
        }
        0b0010_0000 => {
            Ok(16)
        }
        0b0011_0000 => {
            // TODO: Change cause to InvalidValue once it is supported
            Err(PerCodecError::new(PerCodecErrorCause::Generic, "Binary PER decoding of REAL encountered an unexpected reserved value"))
        }
        _ => {
            // TODO: Change cause to InvalidValue once it is supported
            Err(PerCodecError::new(PerCodecErrorCause::Generic, "Binary PER decoding of REAL encountered a software issue"))
        }
    }
}

// ITU X.690 section 8.5.7.3: get the value of the scale factor
fn get_scaling_factor(first_byte: u8) -> u8 {
    // Extract the exponent of the scaling factor
    let scaling_factor_exp = ((first_byte & 0b0000_1100) >> 2) as u32;
    // Raising 2^x is the is the same as 1 << x
    1u8 << scaling_factor_exp
}

// ITU X.690 section 8.5.7.4: get the length in bytes of the encoded exponent
fn get_exponent_length(first_byte: u8, data: &mut PerCodecData, total_length: usize) -> Result<(usize, usize), PerCodecError> {
    let format_value = first_byte & 0b0000_0011;
    if first_byte & 0b0000_0011 == 0b0000_0011 {
        let second_byte = data.decode_bits_as_integer(8, false)?;
        if let Ok(second_byte) = u8::try_from(second_byte) {
            // The second byte contains the number of bytes that
            // the exponent will use
            let exponent_length: usize = second_byte.into();
            let total_length = total_length - 1;

            // Verify that the exponent value fits in the expected total length
            if exponent_length > total_length {
                return Err(PerCodecError::new(PerCodecErrorCause::Generic, "Encoded exponent length too long for reported REAL encoding size"));
            }
            let total_length = total_length - exponent_length;
            Ok((exponent_length, total_length))
        } else {
            return Err(PerCodecError::new(
                PerCodecErrorCause::Generic,
                "Could not convert i128 with 8 data bits into a u8; please contact the developers"
            ));
        }
    } else {
        // The number of bytes in the exponent happens to be the
        // value of the enumerated format + 1 for all values except
        // the last one.
        let exponent_length: usize = (format_value + 1).into();

        // Verify that the exponent value fits in the expected total length
        if exponent_length > total_length {
            return Err(PerCodecError::new(PerCodecErrorCause::Generic, "Encoded exponent length too long for reported REAL encoding size"));
        }
        let total_length = total_length - exponent_length;

        Ok((exponent_length, total_length))
    }
}

// ITU X.690 section 8.5.7.4: get the value of the exponent
fn get_exponent(first_byte: u8, data: &mut PerCodecData, remaining_length: usize) -> Result<(f64, usize), PerCodecError> {
    let (exponent_length, remaining_length) = get_exponent_length(first_byte, data, remaining_length)?;
    let exponent_bytes = data.get_bytes(exponent_length)?;
    let mut exponent = 0.0f64;
    for exponent_byte in exponent_bytes {
        // This is the equivalent of bit-shifting the existing
        // unsigned integer value to the left by 8 and putting
        // the new value in the least significant byte. The
        // difference is that this is being done over a floating
        // point value. Note that the exponent can be negative,
        // so the bytes are encoded as two's complement.
        exponent = exponent * 256.0f64 + (exponent_byte as i8) as f64;
    }
    Ok((exponent, remaining_length))
}

// ITU X.690 section 8.5.7.4: get the value of the mantissa
fn get_mantissa_input(data: &mut PerCodecData, remaining_length: usize) -> Result<f64, PerCodecError> {
    let mantissa_input_bytes = data.get_bytes(remaining_length)?;
    let mut mantissa_input = 0.0f64;
    for mantissa_input_byte in mantissa_input_bytes {
        // This is the equivalent of bit-shifting the existing
        // unsigned integer value to the left by 8 and putting
        // the new value in the least significant byte. The
        // difference is that this is being done over a floating
        // point value.
        mantissa_input = mantissa_input * 256.0f64 + mantissa_input_byte as f64;
    }
    Ok(mantissa_input)
}

// ITU X.690 section 8.5.7: decode the binary value
pub(super) fn decode_real_as_binary(first_byte: u8, data: &mut PerCodecData, length: usize) -> Result<f64, PerCodecError> {
    let sign = get_sign(first_byte) as f64;
    let base = get_base(first_byte)? as f64;
    let scaling_factor = get_scaling_factor(first_byte) as f64;
    let (exponent, remaining_length) = get_exponent(first_byte, data, length)?;
    let mantissa_input = get_mantissa_input(data, remaining_length)?;
    log::trace!(
        "REAL binary components: Sign: {sign}, Base: {base}, Scaling factor: {scaling_factor}, Exponent: {exponent}, N: {mantissa_input}"
    );

    // From the spec: M = S x N x 2^F
    let mantissa = sign * mantissa_input * scaling_factor;

    // The REAL value is mantissa * base^exponent
    Ok(mantissa * base.powf(exponent))
}

// ITU X.690 section 8.5.8: decode the ISO 6093-encoded value
pub(super) fn decode_real_as_decimal(data: &mut PerCodecData, remaining_length: usize) -> Result<f64, PerCodecError> {
    // Decode as base 10 NR1, NR2, or NR3
    if let Ok(decoded_bytes) = data.get_bytes(remaining_length) {
        if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {

            // Replace commas separators with dot separators.
            // The comma is allowed in the ISO 6093 spec, but
            // it cannot be used for conversion in Rust's parse.
            let decoded_str: String = decoded_str.chars()
            .map(|x| match x {
                ',' => '.',
                _ => x
            }).collect();

            // The parse function handles NR1, NR2, and NR3 encoding
            if let Ok(parsed_value) = decoded_str.parse::<f64>() {
                return Ok(parsed_value);
            } else {
                return Err(PerCodecError::new(
                    PerCodecErrorCause::Generic,
                    format!(
                        "Invalid REAL encoded string detected: {}",
                        decoded_str,
                    )
                ));
            }
        } else {
            return Err(PerCodecError::new(
                PerCodecErrorCause::Generic,
                "Unable to decode REAL value as utf8 string"
            ));
        }
    } else {
        return Err(PerCodecError::new(
            PerCodecErrorCause::Generic,
            format!(
                "Unable to get {} bytes to decode REAL value",
                remaining_length,
            )
        ));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_decode_constrained_whole_number_range_0_aligned() {
        let data = &[0x70u8, 0, 0, 0];
        let mut codec_data = PerCodecData::from_slice_aper(data);
        codec_data.advance_maybe_err(1, false).unwrap();
        let value = decode_constrained_whole_number_common(&mut codec_data, 14, 14, true);
        assert!(value.is_ok());
        let value = value.unwrap();
        assert_eq!(value, 14i128);
    }

    #[test]
    fn test_decode_constrained_whole_number_lt_256_aligned() {
        let data = &[0x70u8, 0, 0, 0];
        let mut codec_data = PerCodecData::from_slice_aper(data);
        codec_data.advance_maybe_err(1, false).unwrap();
        let value = decode_constrained_whole_number_common(&mut codec_data, 7, 14, true);
        assert!(value.is_ok());
        let value = value.unwrap();
        assert_eq!(value, 14i128);
    }

    #[test]
    fn test_decode_constrained_whole_number_eq_256_aligned() {
        let data = &[0x80u8, 0x70u8, 0, 0];
        let mut codec_data = PerCodecData::from_slice_aper(data);
        codec_data.advance_maybe_err(1, false).unwrap();
        let value = decode_constrained_whole_number_common(&mut codec_data, 0, 255, true);
        assert!(value.is_ok(), "{:#?}", value.err().unwrap());
        let value = value.unwrap();
        assert_eq!(value, 0x70i128);
    }

    #[test]
    fn test_decode_constrained_whole_number_lt_64k_aligned() {
        let data = &[0x00u8, 0x70u8, 0x00, 1];
        let mut codec_data = PerCodecData::from_slice_aper(data);
        codec_data.advance_maybe_err(12, false).unwrap();
        let value = decode_constrained_whole_number_common(&mut codec_data, 0, 64000, true);
        assert!(value.is_ok(), "{:#?}", value.err().unwrap());
        let value = value.unwrap();
        assert_eq!(value, 1_i128);
    }

    #[test]
    fn test_decode_int_range_68719476735_aligned() {
        let mut data = PerCodecData::from_slice_aper(&[0x00, 0x7B]);
        let value =
            decode_constrained_whole_number_common(&mut data, 0, 68719476735, true).unwrap();
        assert_eq!(value, 123);
    }

    #[test]
    fn test_decode_constrained_whole_number_gt_64k_aligned() {
        let data = &[0x00u8, 0x78u8, 0x01, 1, 0x01, 0x02];
        let mut codec_data = PerCodecData::from_slice_aper(data);
        codec_data.advance_maybe_err(12, false).unwrap();

        // We are now looking at the 12th bit.
        // 0000 0000 0111 >> 1000 0000 0001 0000 0001 0000 0001 0000 0002

        let value = decode_constrained_whole_number_common(&mut codec_data, 0, 20_000_000, true);
        assert!(value.is_ok(), "{:#?}", value.err().unwrap());
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
