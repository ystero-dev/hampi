use asn_codecs_derive::AperCodec;

#[derive(AperCodec)]
struct CodecTest;

fn main() {
    let _ = CodecTest {};
}
