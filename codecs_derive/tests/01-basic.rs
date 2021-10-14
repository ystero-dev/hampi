use asn_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", extensible = true, lb = 0, ub = 2, sz_lb = -5)]
struct CodecTest {}

fn main() {
    let t = CodecTest {};
    eprintln!("t: {:#?}", t);
}
