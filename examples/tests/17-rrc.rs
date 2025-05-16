#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

mod rrc {
    include!(concat!(env!("OUT_DIR"), "/rrc.rs"));
}

fn main() {
    use asn1_codecs::{uper::UperCodec, PerCodecData};
    let _ = env_logger::init();

    // LTE SIB1 value
    let raw_sib1 =
        hex::decode("68CC42821989C402240F3F6BC2D03EA18C80840C22611D0E098FD080814B62E0").unwrap();
    let mut sib1_codec_data = PerCodecData::from_slice_uper(&raw_sib1);
    let sib1 = rrc::BCCH_DL_SCH_Message::uper_decode(&mut sib1_codec_data);
    eprintln!("sib1: {:#?}", sib1.unwrap());

    let mut raw_sib1 = hex::decode("201bbfa806018001400e880110800a409fe2c20a409de2c404748a82620044200c0118443123213086d70578572c2698ac4b45031000200643002a2081989010ba17a389800060300a84c058310002728134000000000ffffffff650d02b1d6143000488201600").unwrap();
    let mut sib1_codec_data = PerCodecData::from_slice_uper(&raw_sib1);
    let sib1 = rrc::DL_DCCH_Message::uper_decode(&mut sib1_codec_data);

    eprintln!("sib1: {:#?}", sib1.unwrap());
}
