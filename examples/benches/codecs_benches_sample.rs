#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

use criterion::{criterion_group, criterion_main, Criterion};

#[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
pub mod sample_world3d {
    include!(concat!(env!("OUT_DIR"), "/world3d.rs"));
}

use asn1_codecs::{uper::UperCodec, PerCodecData}; // HAMPI codec

// Taken from https://github.com/dudycz/asn1_codecs_bench
//
pub fn build_sample_hampi() -> sample_world3d::World {
    use crate::sample_world3d::*;
    World {
        depth: WorldDepth(
            (0..10)
                .map(|_| Plane {
                    rows: PlaneRows(
                        (0..10)
                            .map(|_| Column {
                                elements: ColumnElements(
                                    (0..10)
                                        .map(|_| Color {
                                            r: ColorR(42),
                                            g: ColorG(128),
                                            b: ColorB(77),
                                            a: ColorA(12312),
                                        })
                                        .collect(),
                                ),
                            })
                            .collect(),
                    ),
                })
                .collect(),
        ),
    }
}

fn world3d_encode_bench(c: &mut Criterion) {
    let world = build_sample_hampi();

    c.bench_function("world3d_encode", |b| {
        b.iter(|| {
            let mut codec_encode_data = PerCodecData::new_uper();
            let _ = world.uper_encode(&mut codec_encode_data).unwrap();
        });
    });
}

fn world3d_decode_bench(c: &mut Criterion) {
    let world = build_sample_hampi();
    let mut codec_encode_data = PerCodecData::new_uper();
    let _ = world.uper_encode(&mut codec_encode_data).unwrap();
    let encoded = codec_encode_data.into_bytes();

    c.bench_function("world3d_decode", |b| {
        b.iter(|| {
            let mut codec_decode_data = PerCodecData::from_slice_uper(&encoded);
            let _ = sample_world3d::World::uper_decode(&mut codec_decode_data).unwrap();
        });
    });
}

criterion_group!(
    sample_codecs_world3d,
    world3d_encode_bench,
    world3d_decode_bench,
);

criterion_main!(sample_codecs_world3d);
