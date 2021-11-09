#![allow(non_camel_case_types)]

use asn1_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING")]
pub struct EN_DCSONConfigurationTransfer(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct EPS_TAC(Vec<u8>);

fn main() {
    eprintln!("OCTET STRING");
}
