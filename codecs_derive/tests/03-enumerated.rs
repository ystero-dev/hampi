#![allow(dead_code)]

use asn_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(type = "ENUMERATED", extensible = true, lb = 0i128, ub = 0i128)]
pub struct AdditionalQosFlowInformation(u8);
impl AdditionalQosFlowInformation {
    const MORE_LIKELY: u8 = 0u8;
}

fn main() {
    eprintln!("Enumerated");
}
