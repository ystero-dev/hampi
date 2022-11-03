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
