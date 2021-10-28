use asn_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", lb = 0i128, ub = 255i128)]
pub struct ProcedureCode(u8);

fn main() {
    eprintln!("ProcedureCode");
}
