#![allow(non_camel_case_types)]

use asn_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(
    type = "PrintableString",
    sz_extensible = false,
    sz_lb = 1i128,
    sz_ub = 150i128
)]
pub struct RANNodeName(String);

#[derive(Debug, AperCodec)]
#[asn(
    type = "UTF8String",
    sz_extensible = false,
    sz_lb = 1i128,
    sz_ub = 150i128
)]
pub struct RANNodeNameUTF8String(String);

#[derive(Debug, AperCodec)]
#[asn(
    type = "VisibleString",
    sz_extensible = false,
    sz_lb = 1i128,
    sz_ub = 150i128
)]
pub struct RANNodeNameVisibleString(String);

#[derive(Debug, AperCodec)]
#[asn(type = "VisibleString")]
pub struct URI_address(String);

fn main() {
    eprintln!("Charstring");
}
