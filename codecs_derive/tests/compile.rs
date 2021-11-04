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
    t.pass("tests/07-null.rs");
    t.pass("tests/08-seq.rs");
    t.pass("tests/09-open.rs");
    t.pass("tests/10-seqof.rs");
    // TODO: When we can parse all the three specs enable these.
    //t.pass("tests/11-s1ap.rs");
    //t.pass("tests/12-ngap.rs");
    //t.pass("tests/13-ngap.rs");
}
