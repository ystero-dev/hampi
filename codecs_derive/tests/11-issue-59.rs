#![allow(non_camel_case_types, dead_code)]

use asn1_codecs::{uper::UperCodec, PerCodecData};

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct INTEGER_20(pub u8);

fn main() {
    let mut data: PerCodecData = PerCodecData::new_uper();
    let my_int: INTEGER_20 = INTEGER_20(3);
    let _result = my_int.uper_encode(&mut data);
}
