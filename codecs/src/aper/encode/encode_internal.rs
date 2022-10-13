use crate::aper::encode::encode_length_determinent;
use crate::aper::AperCodecData;
use crate::aper::AperCodecError;
use bitvec::prelude::*;

pub(super) fn encode_unconstrained_whole_number(
    data: &mut AperCodecData,
    value: i128,
) -> Result<(), AperCodecError> {
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
    encode_length_determinent(data, None, None, false, bytes_needed as usize)?;
    let bytes = value.to_be_bytes();
    data.append_bits(bytes[16 - bytes_needed as usize..16].view_bits());
    Ok(())
}

pub(super) fn encode_semi_constrained_whole_number(
    data: &mut AperCodecData,
    lb: i128,
    value: i128,
) -> Result<(), AperCodecError> {
    if value < lb {
        return Err(AperCodecError::new(format!(
            "Cannot encode integer {} - less than lower bound {}",
            value, lb,
        )));
    }

    encode_unconstrained_whole_number(data, value - lb)
}

pub(super) fn encode_constrained_whole_number(
    data: &mut AperCodecData,
    lb: i128,
    ub: i128,
    value: i128,
) -> Result<(), AperCodecError> {
    if value < lb {
        return Err(AperCodecError::new(format!(
            "Cannot encode integer {} - less than lower bound {}",
            value, lb,
        )));
    }

    if value > ub {
        return Err(AperCodecError::new(format!(
            "Cannot encode integer {} - greater than upper bound {}",
            value, ub,
        )));
    }

    let range = ub - lb + 1;
    let value = value - lb;

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
        let bytes_needed_for_range = crate::aper::bytes_needed_for_range(range) as i128;
        let bytes = value.to_be_bytes();
        let first_non_zero = bytes.iter().position(|x| *x != 0).unwrap_or(16);
        encode_constrained_whole_number(
            data,
            1,
            bytes_needed_for_range,
            16 - first_non_zero as i128,
        )?;
        data.align();
        data.append_bits(bytes[first_non_zero..16].view_bits());
    }
    Ok(())
}

pub(super) fn encode_indefinite_length_determinent(
    data: &mut AperCodecData,
    value: usize,
) -> Result<(), AperCodecError> {
    data.align();
    if value < 128 {
        let byte = value as u8;
        data.append_bits(byte.view_bits::<Msb0>());
    } else if value < 16384 {
        let bytes = (value as u16 | 0x8000).to_be_bytes();
        data.append_bits(bytes.view_bits::<Msb0>());
    } else {
        return Err(AperCodecError::new(
            "Length determinent >= 16384 not implemented",
        ));
    }
    Ok(())
}

pub(super) fn encode_normally_small_length_determinent(
    data: &mut AperCodecData,
    value: usize,
) -> Result<(), AperCodecError> {
    if value <= 32 {
        let byte = (value - 1) as u8;
        data.encode_bool(false);
        data.append_bits(&byte.view_bits::<Msb0>()[2..8]);
    } else {
        data.encode_bool(true);
        encode_indefinite_length_determinent(data, value)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn encode_unconstrained() {
        let mut data = AperCodecData::new();
        encode_unconstrained_whole_number(&mut data, 1).unwrap();
        assert_eq!(data.into_bytes(), [0x01, 0x01]);
    }

    #[test]
    fn encode_tiny_constrained_integer() {
        let mut data = AperCodecData::new();
        encode_constrained_whole_number(&mut data, -2, 5, -1).unwrap();
        assert_eq!(data.into_bytes(), [0x20]);
    }

    #[test]
    fn encode_small_constrained_integer() {
        let mut data = AperCodecData::new();
        encode_constrained_whole_number(&mut data, 0, 1000, 1).unwrap();
        assert_eq!(data.into_bytes(), [0x00, 0x01]);
    }

    #[test]
    fn encode_large_constrained_integer() {
        let mut data = AperCodecData::new();
        encode_constrained_whole_number(&mut data, 0, 100_000, 1).unwrap();
        assert_eq!(data.into_bytes(), [0x00, 0x01]);
    }

    #[test]
    fn encode_tiny_indefinite_length_determinent() {
        let mut data = AperCodecData::new();
        encode_indefinite_length_determinent(&mut data, 1).unwrap();
        assert_eq!(data.into_bytes(), [0x01]);
    }

    #[test]
    fn encode_small_indefinite_length_determinent() {
        let mut data = AperCodecData::new();
        encode_indefinite_length_determinent(&mut data, 16383).unwrap();
        assert_eq!(data.into_bytes(), [0xbf, 0xff]);
    }

    #[test]
    fn encode_small_normally_small_length_determinent() {
        let mut data = AperCodecData::new();
        encode_normally_small_length_determinent(&mut data, 32).unwrap();
        assert_eq!(data.into_bytes(), [0x3e]);
    }

    #[test]
    fn encode_int_range_256() {
        let mut data = AperCodecData::new();
        data.encode_bool(true);
        encode_constrained_whole_number(&mut data, 0, 255, 1).unwrap();
        assert_eq!(data.into_bytes(), [0x80, 0x01]);
    }

    #[test]
    fn encode_int_range_68719476735() {
        let mut data = AperCodecData::new();
        encode_constrained_whole_number(&mut data, 0, 68719476735, 123).unwrap();
        assert_eq!(data.into_bytes(), [0x00, 0x7B]);
    }

    #[test]
    fn encode_u32() {
        let mut data = AperCodecData::new();
        let value = 0x10203040;
        encode_constrained_whole_number(&mut data, 0, 4294967295, value).unwrap();
        assert_eq!(data.into_bytes(), [0xC0, 0x10, 0x20, 0x30, 0x40]);
    }
}
