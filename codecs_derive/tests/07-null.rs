#![allow(non_camel_case_types)]

use asn1_codecs_derive::{AperCodec, UperCodec};

#[derive(Debug, AperCodec, UperCodec)]
#[asn(type = "NULL")]
pub struct NULL_3;

fn main() {
    eprintln!("Null");
}
