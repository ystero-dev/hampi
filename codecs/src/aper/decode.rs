use crate::aper::AperCodecData;
use crate::aper::AperCodecError;

pub fn decode_choice_idx(lb: i128, ub: i128) -> Result<(), AperCodecError> {
    Ok(())
}

fn decode_integer_no_ext_marker_in_constraint() {}

fn decode_integer_ext_marker_in_constraints() {}

// Section 10.8 X.691
fn decode_unconstrained_whole_number() {}

fn decode_semi_constrained_whole_number() {}

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
            let value = data.decode_bits(bits);
            Ok(value)
        } else if range == 256 {
            let _ = data.decode_align()?;
            let value = data.decode_bits(8);
            Ok(value)
        } else if range < 65536 {
            let _ = data.decode_align()?;
            let value = data.decode_bits(16);
            Ok(value)
        } else {
            Err(AperCodecError::new(
                format!("Range {} Not supported!", range).as_str(),
            ))
        };

        value.unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_decode_constrained_whole_number_lt_256() {
        let data = &[0x70u8, 0, 0, 0];
        let mut codec_data = AperCodecData::from_u8(data);
        codec_data.advance(1);
        assert!(true);
        let value = decode_constrained_whole_number(&mut codec_data, 7, 14);
        assert!(value.is_ok());
        let value = value.unwrap();
        assert_eq!(value, 7i128);
    }

    #[test]
    fn test_decode_constrained_whole_number_eq_256() {
        let data = &[0x80u8, 0x70u8, 0, 0];
        let mut codec_data = AperCodecData::from_u8(data);
        codec_data.advance(1);
        assert!(true);
        let value = decode_constrained_whole_number(&mut codec_data, 0, 255);
        assert!(value.is_ok(), "{:#?}", value.err());
        let value = value.unwrap();
        assert_eq!(value, 0x70i128);
    }

    #[test]
    fn test_decode_constrained_whole_number_gt_256() {
        let data = &[0x00u8, 0x70u8, 0x00, 1];
        let mut codec_data = AperCodecData::from_u8(data);
        codec_data.advance(12);
        assert!(true);
        let value = decode_constrained_whole_number(&mut codec_data, 0, 64000);
        assert!(value.is_ok(), "{:#?}", value.err());
        let value = value.unwrap();
        assert_eq!(value, 1_i128);
    }
}
