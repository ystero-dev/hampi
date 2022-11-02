#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

mod ranap {
    include!(concat!(env!("OUT_DIR"), "/ranap.rs"));
}

fn main() {
    use asn1_codecs::aper::*;
    eprintln!("RANAP");

    let ranap_data = hex::decode("000100080000010004400122").unwrap();
    let mut codec_data = AperCodecData::from_slice(&ranap_data);
    let ranap_pdu = ranap::RANAP_PDU::aper_decode(&mut codec_data).unwrap();
    eprintln!("ranap_pdu: {:#?}", ranap_pdu);

    let ranap_data = hex::decode("0014400f0000020010400302832a003b400100").unwrap();
    let mut codec_data = AperCodecData::from_slice(&ranap_data);
    let ranap_pdu = ranap::RANAP_PDU::aper_decode(&mut codec_data).unwrap();
    eprintln!("ranap_pdu: {:#?}", ranap_pdu);
}
