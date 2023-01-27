use serde::Deserialize;
use std::{env, error::Error, path::Path};

#[derive(Deserialize)]
pub struct TsvRow {
    lhs: String,
    relation: String,
    rhs: String,
}

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=dictionaries/source/kVariants.tsv");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    compress_kvariant_txt().unwrap();
}

fn compress_kvariant_txt() -> Result<(), Box<dyn Error>> {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let src_path = Path::new(&manifest_dir).join("dictionaries/source/kVariants.tsv");
    let mut reader =
        csv::ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_path(src_path)?;

    let dest_path = Path::new(&manifest_dir).join("dictionaries/compressed/kVariants.csv");
    let mut writer = csv::Writer::from_path(dest_path)?;

    for result in reader.deserialize() {
        let line: TsvRow = result?;

        // Extract "㨲" from "㨲 (U+3A32)"
        let rhs = line.rhs.chars().next().unwrap();
        let lhs = line.lhs.chars().next().unwrap();

        writer.write_record(&[lhs.to_string(), line.relation, rhs.to_string()])?;
    }

    writer.flush()?;
    Ok(())
}
