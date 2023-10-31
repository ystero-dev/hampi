#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

mod e2sm {
    include!(concat!(env!("OUT_DIR"), "/e2sm.rs"));
}

fn main() {
    use asn1_codecs::{aper::AperCodec, PerCodecData};
    eprintln!("E2SM");

    let _ = env_logger::init();

    // This will break: Fixed
    let decode_str = "080000000100000000002043514901200000";

    let decode_hex = hex::decode(decode_str).unwrap();
    let mut codec_data = PerCodecData::from_slice_aper(&decode_hex);
    let e2sm_kpmv3_pdu = e2sm::E2SM_KPM_IndicationMessage::aper_decode(&mut codec_data);
    let e2sm_kpmv3_pdu = e2sm_kpmv3_pdu.unwrap();
    eprintln!("e2sm_kpmv3_pdu: {:#?}", e2sm_kpmv3_pdu);

    let mut encode_codec_data = PerCodecData::new_aper();
    let result = e2sm_kpmv3_pdu.aper_encode(&mut encode_codec_data);
    eprintln!("result: {:#?}", result);

    let e2sm_kpmv3_encoded_data = encode_codec_data.get_inner().unwrap();
    eprintln!("Encoded: {}", hex::encode(&e2sm_kpmv3_encoded_data));
    eprintln!("Original: {}", decode_str);

    let e2sm_kpmv3_pdu = e2sm::E2SM_KPM_IndicationMessage::aper_decode(&mut encode_codec_data);
    let e2sm_kpmv3_pdu = e2sm_kpmv3_pdu.unwrap();
    eprintln!("e2sm_kpmv3_pdu: {:#?}", e2sm_kpmv3_pdu);
}
