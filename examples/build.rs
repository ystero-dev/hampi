//! An Example of using ASN.1 compiler in `build.rs`
//!

use std::env;
use std::path::PathBuf;

use asn1_compiler::{
    generator::{Codec, Derive, Visibility},
    Asn1Compiler,
};

fn main() -> std::io::Result<()> {
    let ranap_specs_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("specs")
        .join("ranap");
    eprintln!("{:#?}", ranap_specs_dir);

    let ranap_spec_files = ranap_specs_dir
        .read_dir()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().file_name().is_some())
        .filter(|e| {
            e.path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("RANAP")
        })
        .map(|e| e.path())
        .collect::<Vec<_>>();
    eprintln!("ranap_spec_files: {:#?}", ranap_spec_files);

    let ranap_rs_module = PathBuf::from(env::var("OUT_DIR").unwrap()).join("ranap.rs");
    let ranap_rs_module = ranap_rs_module.to_str().unwrap();
    let mut compiler = Asn1Compiler::new(
        ranap_rs_module,
        false,
        &Visibility::Public,
        vec![Codec::Aper],
        vec![Derive::Debug, Derive::Serialize, Derive::Deserialize],
    );

    compiler.compile_files(&ranap_spec_files)?;

    Ok(())
}
