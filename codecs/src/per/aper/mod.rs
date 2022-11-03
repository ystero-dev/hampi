#![allow(dead_code)]
//! ASN.1 Aligned PER Codec

pub mod encode;

pub mod decode;

pub use crate::per::error::Error as AperCodecError;
pub use crate::per::PerCodecData;
pub use crate::per::PerCodecData as AperCodecData;

/// Trait representing an 'APER Codec'.
///
///
/// This 'trait' is to be derived by any `struct` or `enum` representing an ASN.1 Type.
pub trait AperCodec {
    type Output;

    fn aper_decode(data: &mut PerCodecData) -> Result<Self::Output, AperCodecError>;

    fn aper_encode(&self, _data: &mut PerCodecData) -> Result<(), AperCodecError> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::prelude::*;

    // A test that would fail if it were not for the `force_align()` in PerCodecData::get_bytes().
    #[test]
    fn get_bytes_unaligned() {
        let mut d = PerCodecData::from_slice(&vec![0x0f, 0xf0]);
        let _ = d.get_bitvec(4);
        let bytes = d.get_bytes(1).unwrap();
        assert_eq!(bytes, vec![0xff]);
    }

    #[test]
    fn test_encode_decode_unconstrained_whole_number() {
        let numbers: Vec<i128> = vec![
            140737488355328,
            140737488355327,
            549755813888,
            549755813887,
            2147483648,
            2147483647,
            8388608,
            8388607,
            32768,
            32767,
            128,
            127,
            1,
            0,
            -1,
            -128,
            -129,
            -32768,
            -32769,
            -8388608,
            -8388609,
            -2147483648,
            -2147483649,
            -549755813888,
            -549755813889,
            -140737488355328,
            -140737488355329,
        ];
        //let numbers: Vec<i128> = vec![-256, -1, -65537, 0, 11, 127, 128, 65536, 1234567, 123456789];
        for num in numbers {
            let mut d = PerCodecData::new();
            eprintln!("number: {}", num);
            let result = encode::encode_integer(&mut d, None, None, false, num, false);
            eprintln!("{:?}", d);
            assert!(result.is_ok(), "{:#?}", d);
            let value = decode::decode_integer(&mut d, None, None, false);
            assert!(value.is_ok(), "{:#?}", value.err());
            assert!(value.unwrap().0 == num);
        }
    }

    // Proves get_bitvec() can cope if it is asked for all the remaining bits in the buffer.
    #[test]
    fn get_all_remaining_bits() {
        let mut d = PerCodecData::new();
        d.append_bits(bits![u8, Msb0; 1,0,1,0]);
        assert_eq!(d.get_bitvec(4).unwrap(), bitvec![u8,Msb0;1,0,1,0]);
    }

    // Likewise for get_bytes().
    #[test]
    fn get_all_remaining_bytes() {
        let mut d = PerCodecData::new();
        let b: u8 = 0b10101111;
        d.append_bits(b.view_bits());
        assert_eq!(d.get_bytes(1).unwrap()[0], b);
    }

    #[test]
    fn printable_string_coding() {
        let mut d = PerCodecData::new();
        let s1 = "hello".to_string();
        encode::encode_printable_string(&mut d, None, None, false, &s1, false).unwrap();
        let s2 = decode::decode_printable_string(&mut d, None, None, false).unwrap();
        assert_eq!(s1, s2);
    }

    #[test]
    fn empty_string() {
        let mut d = PerCodecData::new();
        let s1 = "".to_string();
        encode::encode_printable_string(&mut d, None, None, false, &s1, false).unwrap();
        let s2 = decode::decode_printable_string(&mut d, None, None, false).unwrap();
        assert_eq!(s1, s2);
    }
}
