use criterion::{criterion_group, criterion_main, Criterion};

use asn1_codecs::aper::{AperCodec, AperCodecData};
mod ngap;
use ngap::NGAP_PDU;

pub fn ngap_decode_bench(c: &mut Criterion) {
    let ngap_data = hex::decode(
        "0015404a000004001b00084002f898000000000052400f06004d79206c6974746c6520674e420066001f01000000000002f8980001000800800000010002f8390001001881c00013880015400140",
    ).unwrap();

    c.bench_function("NGAP_decode", |b| {
        b.iter(|| {
            let mut codec_data = AperCodecData::from_slice(&ngap_data);
            let _ = NGAP_PDU::decode(&mut codec_data).unwrap();
        });
    });
}

criterion_group!(ngap_decode, ngap_decode_bench);
criterion_main!(ngap_decode);
