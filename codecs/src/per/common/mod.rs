pub(crate) mod encode;

pub(crate) mod decode;

// FIXME: Remove the pub(crate) when `decode` also pulled uner `common`.
pub(crate) fn bytes_needed_for_range(range: i128) -> u8 {
    let bits_needed: u8 = 128 - (range - 1).leading_zeros() as u8;
    let mut bytes_needed = bits_needed / 8;
    if bits_needed % 8 != 0 {
        bytes_needed += 1
    }
    bytes_needed
}

// See X.690 section 8.5.9 for details on special REAL values
const NEGATIVE_ZERO: u8 = 0b0100_0011;
const INFINITY: u8 = 0b0100_0000;
const NEGATIVE_INFINITY: u8 = 0b0100_0001;
const NOT_A_NUMBER: u8 = 0b0100_0010;
const BASE_10_NR1: u8 = 0b0000_0001;
const BASE_10_NR2: u8 = 0b0000_0010;
const BASE_10_NR3: u8 = 0b0000_0011;
