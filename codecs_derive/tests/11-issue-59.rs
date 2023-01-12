#![allow(non_camel_case_types, dead_code)]

use asn1_codecs::{uper::UperCodec, PerCodecData};

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct INTEGER_20(pub u8);

fn main() {
    let mut data = PerCodecData::new_uper();
    let my_int: INTEGER_20 = INTEGER_20(3);
    let result = my_int.uper_encode(&mut data);
    assert!(result.is_ok());

    let decoded = INTEGER_20::uper_decode(&mut data);
    assert!(decoded.is_ok(), "{:?}", decoded.err().unwrap());

    let INTEGER_20(v) = decoded.unwrap();
    assert!(v == 3u8, "decoded: {}", v);
}
