#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-choice.rs");
    t.pass("tests/02-integer.rs");
    t.pass("tests/03-enumerated.rs");
    t.pass("tests/04-bitstring.rs");
    t.pass("tests/05-octetstring.rs");
    t.pass("tests/06-charstring.rs");
}
