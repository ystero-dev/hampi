#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/11-ranap.rs");
    t.pass("tests/12-s1ap.rs");
    t.pass("tests/13-ngap.rs");
    t.pass("tests/14-e2ap.rs");
}
