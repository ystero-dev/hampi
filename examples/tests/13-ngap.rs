#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

mod ngap {
    include!(concat!(env!("OUT_DIR"), "/ngap.rs"));
}

fn main() {
    use asn1_codecs::{aper::AperCodec, PerCodecData};
    eprintln!("NGAP");

    let _ = env_logger::init();

    let ngap_byte_str = "0015404a000004001b00084002f898000000000052400f06004d79206c6974746c6520674e420066001f01000000000002f8980001000800800000010002f8390001001881c00013880015400140";
    let ngap_data = hex::decode(ngap_byte_str).unwrap();
    let mut codec_data = PerCodecData::from_slice_aper(&ngap_data);
    let ngap_pdu = ngap::NGAP_PDU::aper_decode(&mut codec_data).unwrap();

    eprintln!("ngap_pdu: {:#?}", ngap_pdu);
    let mut encode_codec_data = PerCodecData::new_aper();
    let result = ngap_pdu.aper_encode(&mut encode_codec_data);
    eprintln!("result: {:#?}", result);
    let ngap_encoded_data = encode_codec_data.get_inner().unwrap();
    eprintln!("Original: {}", hex::encode(&ngap_encoded_data));
    eprintln!("Encoded: {}", ngap_byte_str);
    eprintln!("{}", ngap_data.len() == ngap_encoded_data.len());
    let ngap_pdu = ngap::NGAP_PDU::aper_decode(&mut encode_codec_data);
    eprintln!("ngap_pdu: {:#?}", ngap_pdu);

    // Error response!
    let response_data = [
        32, 21, 0, 48, 0, 0, 4, 0, 21, 0, 12, 4, 128, 116, 97, 120, 105, 108, 97, 45, 97, 109, 102,
        0, 96, 0, 8, 0, 0, 9, 241, 7, 1, 2, 191, 0, 80, 0, 8, 0, 9, 241, 7, 0, 0, 0, 8, 0, 86, 0,
        1, 255,
    ];
    let mut codec_data = PerCodecData::from_slice_aper(&response_data);
    let ngap_pdu = ngap::NGAP_PDU::aper_decode(&mut codec_data);
    eprintln!("ngap_pdu: {:?}", ngap_pdu);
}
