use asn_codecs_derive::AperCodec;

#[derive(Debug, AperCodec)]
#[asn(type = "INTEGER", extensible = true, lb = 0, ub = 2, sz_lb = -5)]
struct X(u8);
impl X {
    fn decode<T>(_data: T) -> Self {
        X(0_u8)
    }
}

#[derive(Debug, AperCodec)]
#[asn(type = "CHOICE", extensible = true, lb = 0, ub = 2, sz_lb = -5)]
enum TestEnum {
    #[asn(key = 0, extended = true)]
    A(X),
}

fn main() {
    eprintln!("t:");
}
