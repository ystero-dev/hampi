#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

use criterion::{criterion_group, criterion_main, Criterion};

use asn1_codecs::{aper::AperCodec, PerCodecData};

mod ngap {
    include!(concat!(env!("OUT_DIR"), "/ngap.rs"));
}

pub fn ngap_decode_bench(c: &mut Criterion) {
    let ngap_data = hex::decode(
        "0015404a000004001b00084002f898000000000052400f06004d79206c6974746c6520674e420066001f01000000000002f8980001000800800000010002f8390001001881c00013880015400140",
    ).unwrap();

    c.bench_function("NGAP_decode", |b| {
        b.iter(|| {
            let mut codec_data = PerCodecData::from_slice_aper(&ngap_data);
            let _ = ngap::NGAP_PDU::aper_decode(&mut codec_data).unwrap();
        });
    });
}

pub fn ngap_codec_roundtrip_bench(c: &mut Criterion) {
    let ngap_data = hex::decode(
        "0015404a000004001b00084002f898000000000052400f06004d79206c6974746c6520674e420066001f01000000000002f8980001000800800000010002f8390001001881c00013880015400140",
    ).unwrap();

    c.bench_function("NGAP_codec_roundtrip", |b| {
        b.iter(|| {
            let mut codec_data = PerCodecData::from_slice_aper(&ngap_data);
            let ngap_pdu = ngap::NGAP_PDU::aper_decode(&mut codec_data).unwrap();

            let mut codec_encode_data = PerCodecData::new_aper();
            let _ = ngap_pdu.aper_encode(&mut codec_encode_data).unwrap();
        });
    });
}

pub fn ngap_encode_bench(c: &mut Criterion) {
    let ngap_data = hex::decode(
        "0015404a000004001b00084002f898000000000052400f06004d79206c6974746c6520674e420066001f01000000000002f8980001000800800000010002f8390001001881c00013880015400140",
    ).unwrap();

    let mut codec_data = PerCodecData::from_slice_aper(&ngap_data);
    let ngap_pdu = ngap::NGAP_PDU::aper_decode(&mut codec_data).unwrap();

    c.bench_function("NGAP_encode", |b| {
        b.iter(|| {
            let mut codec_encode_data = PerCodecData::new_aper();
            let _ = ngap_pdu.aper_encode(&mut codec_encode_data).unwrap();
        });
    });
}

criterion_group!(
    ngap_codec,
    ngap_encode_bench,
    ngap_decode_bench,
    ngap_codec_roundtrip_bench
);
criterion_main!(ngap_codec);
