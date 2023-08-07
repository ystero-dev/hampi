#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

mod e2sm {
    include!(concat!(env!("OUT_DIR"), "/e2sm.rs"));
}

fn main() {
    eprintln!("E2SM");
}
