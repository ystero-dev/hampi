#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

mod s1ap {
    include!(concat!(env!("OUT_DIR"), "/e2ap.rs"));
}

fn main() {
    eprintln!("E2AP");
}
