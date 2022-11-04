use asn1_codecs_derive::{AperCodec, UperCodec};

#[derive(Debug, AperCodec, UperCodec)]
#[asn(type = "INTEGER")]
struct X(u16);

#[derive(Debug, AperCodec, UperCodec)]
#[asn(type = "CHOICE", extensible = true, lb = "0", ub = "2", sz_lb = "0")]
enum TestEnum {
    #[asn(key = 0, extended = true)]
    A(X),
}

fn main() {
    eprintln!("Choice");
}
