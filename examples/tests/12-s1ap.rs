#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

mod s1ap {
    include!(concat!(env!("OUT_DIR"), "/s1ap.rs"));
}

fn main() {
    use asn1_codecs::{aper::AperCodec, PerCodecData};

    let decode_str = "0004001A00000300000005C0098134A200080004800411EA000240020000";
    let decode_hex = hex::decode(decode_str).unwrap();
    let mut codec_data = PerCodecData::from_slice_aper(&decode_hex);
    let s1ap_pdu = s1ap::S1AP_PDU::aper_decode(&mut codec_data);

    eprintln!("s1ap_pdu: {:#?}", s1ap_pdu.unwrap());
}
