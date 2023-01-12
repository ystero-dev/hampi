use crate::per::{PerCodecData, PerCodecError};

use bitvec::prelude::*;

// Refer to section 10.8
pub(super) fn encode_unconstrained_whole_number_common(
    data: &mut PerCodecData,
    value: i128,
    aligned: bool,
) -> Result<(), PerCodecError> {
    let bytes_needed = if value < 0 {
        let leading_ones = value.leading_ones();

        if leading_ones % 8 == 0 {
            16 - leading_ones / 8 + 1
        } else {
            16 - leading_ones / 8
        }
    } else {
        let leading_zeroes = value.leading_zeros();
        if leading_zeroes % 8 == 0 {
            16 - leading_zeroes / 8 + 1
        } else {
            16 - leading_zeroes / 8
        }
    };
    // The alignement is performed inside `encode_length_determinent_common` when `aligned` is
    // `true`.
    super::encode_length_determinent_common(
        data,
        None,
        None,
        false,
        bytes_needed as usize,
        aligned,
    )?;
    let bytes = value.to_be_bytes();
    data.append_bits(bytes[16 - bytes_needed as usize..16].view_bits());
    Ok(())
}

// Refer to section 10.7
pub(super) fn encode_semi_constrained_whole_number_common(
    data: &mut PerCodecData,
    lb: i128,
    value: i128,
    aligned: bool,
) -> Result<(), PerCodecError> {
    if value < lb {
        return Err(PerCodecError::new(format!(
            "Cannot encode integer {} - less than lower bound {}",
            value, lb,
        )));
    }

    encode_unconstrained_whole_number_common(data, value - lb, aligned)
}

// Refer to Section 10.5.6 when `aligned` is `false` and 10.5.7 when `aligned` is `true`
pub(super) fn encode_constrained_whole_number_common(
    data: &mut PerCodecData,
    lb: i128,
    ub: i128,
    value: i128,
    aligned: bool,
) -> Result<(), PerCodecError> {
    if value < lb {
        return Err(PerCodecError::new(format!(
            "Cannot encode integer {} - less than lower bound {}",
            value, lb,
        )));
    }

    if value > ub {
        return Err(PerCodecError::new(format!(
            "Cannot encode integer {} - greater than upper bound {}",
            value, ub,
        )));
    }

    let range = ub - lb + 1;
    let value = value - lb;

    if aligned {
        if range <= 256 {
            let byte = value as u8;
            let bits = match range {
                1 => 0,
                2 => 1,
                3..=4 => 2,
                5..=8 => 3,
                9..=16 => 4,
                17..=32 => 5,
                33..=64 => 6,
                65..=128 => 7,
                129..=255 => 8,
                256 => {
                    data.align();
                    8
                }
                _ => unreachable!(),
            };
            data.append_bits(&byte.view_bits::<Msb0>()[(8 - bits)..8]);
        } else if range <= 65536 {
            data.align();
            let bytes = (value as u16).to_be_bytes();
            data.append_bits(bytes.view_bits::<Msb0>());
        } else {
            let bytes_needed_for_range = crate::per::common::bytes_needed_for_range(range) as i128;
            let bytes = value.to_be_bytes();
            let first_non_zero = bytes.iter().position(|x| *x != 0).unwrap_or(16);
            encode_constrained_whole_number_common(
                data,
                1,
                bytes_needed_for_range,
                16 - first_non_zero as i128,
                aligned,
            )?;
            data.align();
            data.append_bits(bytes[first_non_zero..16].view_bits());
        }
    } else {
        if range > 1 {
            // Minimum number of bits required to encode length is calculated as follows
            // Let's say range is 100 -> 0b_0000_0000_0110_0100 (Minimum number of bits needed is 7)
            // Let's say range is 1000 -> 0b_0000_0011_1110_1000 (Minimum number of bits needed is 10)

            let leading_zeros = (range - 1).leading_zeros() as usize;
            let bytes = value.to_be_bytes();
            data.append_bits(&bytes.view_bits()[leading_zeros..])
        }
    }
    Ok(())
}

// Section 10.9 Note 2. Actual procedure in 10.9.3.6 -> 10.9.3.8
pub(super) fn encode_indefinite_length_determinent_common(
    data: &mut PerCodecData,
    value: usize,
    aligned: bool,
) -> Result<(), PerCodecError> {
    if aligned {
        data.align();
    }
    if value < 128 {
        let byte = value as u8;
        data.append_bits(byte.view_bits::<Msb0>());
    } else if value < 16384 {
        let bytes = (value as u16 | 0x8000).to_be_bytes();
        data.append_bits(bytes.view_bits::<Msb0>());
    } else {
        return Err(PerCodecError::new(
            "Length determinent >= 16384 not implemented",
        ));
    }
    Ok(())
}

pub(super) fn encode_normally_small_length_determinent_common(
    data: &mut PerCodecData,
    value: usize,
    aligned: bool,
) -> Result<(), PerCodecError> {
    if value <= 32 {
        let byte = (value - 1) as u8;
        data.encode_bool(false);
        data.append_bits(&byte.view_bits::<Msb0>()[2..8]);
    } else {
        data.encode_bool(true);
        encode_indefinite_length_determinent_common(data, value, aligned)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn encode_unconstrained_aligned() {
        let mut data = PerCodecData::new_aper();
        encode_unconstrained_whole_number_common(&mut data, 1, true).unwrap();
        assert_eq!(data.into_bytes(), [0x01, 0x01]);
    }

    #[test]
    fn encode_tiny_constrained_integer_aligned() {
        let mut data = PerCodecData::new_aper();
        encode_constrained_whole_number_common(&mut data, -2, 5, -1, true).unwrap();
        assert_eq!(data.into_bytes(), [0x20]);
    }

    #[test]
    fn encode_small_constrained_integer_aligned() {
        let mut data = PerCodecData::new_aper();
        encode_constrained_whole_number_common(&mut data, 0, 1000, 1, true).unwrap();
        assert_eq!(data.into_bytes(), [0x00, 0x01]);
    }

    #[test]
    fn encode_small_constrained_integer_aligned_range_0() {
        let mut data = PerCodecData::new_aper();
        encode_constrained_whole_number_common(&mut data, 1000, 1000, 1000, true).unwrap();
        assert_eq!(data.into_bytes(), []);
    }

    #[test]
    fn encode_small_constrained_integer_unaligned_range_0() {
        let mut data = PerCodecData::new_aper();
        encode_constrained_whole_number_common(&mut data, 1000, 1000, 1000, false).unwrap();
        assert_eq!(data.into_bytes(), []);
    }

    #[test]
    fn encode_small_constrained_integer_unaligned_range_non_0() {
        let mut data = PerCodecData::new_aper();
        encode_constrained_whole_number_common(&mut data, 0, 1, 1, false).unwrap();
        assert_eq!(data.into_bytes(), [0x80]);
    }

    #[test]
    fn encode_small_constrained_integer_aligned_range_0_higher() {
        let mut data = PerCodecData::new_aper();
        let result = encode_constrained_whole_number_common(&mut data, 1000, 1000, 1001, true);
        assert!(result.is_err(), "{:#?}", result.ok().unwrap());
    }

    #[test]
    fn encode_large_constrained_integer_aligned() {
        let mut data = PerCodecData::new_aper();
        encode_constrained_whole_number_common(&mut data, 0, 100_000, 1, true).unwrap();
        assert_eq!(data.into_bytes(), [0x00, 0x01]);
    }

    #[test]
    fn encode_tiny_indefinite_length_determinent_aligned() {
        let mut data = PerCodecData::new_aper();
        encode_indefinite_length_determinent_common(&mut data, 1, true).unwrap();
        assert_eq!(data.into_bytes(), [0x01]);
    }

    #[test]
    fn encode_small_indefinite_length_determinent_aligned() {
        let mut data = PerCodecData::new_aper();
        encode_indefinite_length_determinent_common(&mut data, 16383, true).unwrap();
        assert_eq!(data.into_bytes(), [0xbf, 0xff]);
    }

    #[test]
    fn encode_small_normally_small_length_determinent_aligned() {
        let mut data = PerCodecData::new_aper();
        encode_normally_small_length_determinent_common(&mut data, 32, true).unwrap();
        assert_eq!(data.into_bytes(), [0x3e]);
    }

    #[test]
    fn encode_int_range_256_aligned() {
        let mut data = PerCodecData::new_aper();
        data.encode_bool(true);
        encode_constrained_whole_number_common(&mut data, 0, 255, 1, true).unwrap();
        assert_eq!(data.into_bytes(), [0x80, 0x01]);
    }

    #[test]
    fn encode_int_range_68719476735_aligned() {
        let mut data = PerCodecData::new_aper();
        encode_constrained_whole_number_common(&mut data, 0, 68719476735, 123, true).unwrap();
        assert_eq!(data.into_bytes(), [0x00, 0x7B]);
    }

    #[test]
    fn encode_u32_aligned() {
        let mut data = PerCodecData::new_aper();
        let value = 0x10203040;
        encode_constrained_whole_number_common(&mut data, 0, 4294967295, value, true).unwrap();
        assert_eq!(data.into_bytes(), [0xC0, 0x10, 0x20, 0x30, 0x40]);
    }
}
