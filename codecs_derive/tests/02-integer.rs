use asn_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ProcedureCode(u8);

fn main() {
    eprintln!("Integer");
}
