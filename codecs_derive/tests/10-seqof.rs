#![allow(non_camel_case_types, dead_code)]

use asn1_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "32"
)]
pub struct WLANName(Vec<u8>);

#[derive(Debug, AperCodec)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
struct WLANMeasConfigNameList(Vec<WLANName>);

fn main() {
    eprintln!("SequenceOf");
}
