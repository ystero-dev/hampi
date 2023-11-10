use asn1_codecs_derive::{AperCodec, UperCodec};

#[derive(Debug, AperCodec, UperCodec)]
#[asn(type = "REAL")]
pub struct Measurement(f64);

fn main() {
    eprintln!("Real");
}
