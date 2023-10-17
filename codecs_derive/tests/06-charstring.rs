#![allow(non_camel_case_types)]

use asn1_codecs_derive::{AperCodec, UperCodec};

#[derive(Debug, AperCodec, UperCodec)]
#[asn(
    type = "PrintableString",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct RANNodeName(String);

#[derive(Debug, AperCodec, UperCodec)]
#[asn(type = "UTCTime")]
pub struct Timestamp(String);

#[derive(Debug, AperCodec)]
#[asn(type = "UTF8String", sz_extensible = false, sz_lb = "1", sz_ub = "150")]
pub struct RANNodeNameUTF8String(String);

#[derive(Debug, AperCodec)]
#[asn(
    type = "VisibleString",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct RANNodeNameVisibleString(String);

#[derive(Debug, AperCodec)]
#[asn(type = "VisibleString")]
pub struct URI_address(String);

fn main() {
    eprintln!("Charstring");
}
