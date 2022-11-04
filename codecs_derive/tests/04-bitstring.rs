use asn1_codecs_derive::{AperCodec, UperCodec};

use bitvec::prelude::*;

#[derive(Debug, AperCodec, UperCodec)]
#[asn(type = "BITSTRING", sz_extensible = true, sz_lb = "16", sz_ub = "16")]
pub struct NRencryptionAlgorithms(BitVec<u8, Msb0>);

fn main() {
    eprintln!("BIT STRING");
}
