#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

mod s1ap {
    include!(concat!(env!("OUT_DIR"), "/s1ap.rs"));
}

fn main() {
    eprintln!("S1AP");
}
