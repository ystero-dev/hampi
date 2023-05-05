#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

mod rrc {
    include!(concat!(env!("OUT_DIR"), "/rrc.rs"));
}

fn main() {
    use asn1_codecs::{uper::UperCodec, PerCodecData};
    let _ = env_logger::init();

    // LTE SIB1 value
    let raw_sib1 = hex::decode("68CC42821989C402240F3F6BC2D03EA18C80840C22611D0E098FD080814B62E0").unwrap();
    let mut sib1_codec_data = PerCodecData::from_slice_uper(&raw_sib1);
    let sib1 = rrc::BCCH_DL_SCH_Message::uper_decode(&mut sib1_codec_data);

    eprintln!("sib1: {:#?}", sib1.unwrap());
}
