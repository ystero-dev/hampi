#![allow(non_camel_case_types)]

use asn_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct EN_DCSONConfigurationTransfer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = 2i128,
    sz_ub = 2i128
)]
pub struct EPS_TAC(Vec<u8>);

fn main() {
    eprintln!("OCTET STRING");
}
