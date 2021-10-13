use asn_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", value_extensible = true, lb = 0, ub = 2)]
struct CodecTest;

fn main() {
    let t = CodecTest {};
    eprintln!("t: {:#?}", t);
}
