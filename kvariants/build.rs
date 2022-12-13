use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
pub struct TsvRow {
    lhs: String,
    relation: String,
    rhs: String,
}

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=dictionaries/source/kVariants.tsv");

    compress_kvariant_txt().unwrap();
}

fn compress_kvariant_txt() -> Result<(), Box<dyn Error>> {
    let reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path("dictionaries/source/kVariants.tsv");

    let mut writer = csv::Writer::from_path("dictionaries/compressed/kVariants.csv")?;

    for result in reader?.deserialize() {
        let line: TsvRow = result?;

        // Extract "㨲" from "㨲 (U+3A32)"
        let rhs = line.rhs.chars().next().unwrap();
        let lhs = line.lhs.chars().next().unwrap();

        writer.write_record(&[lhs.to_string(), line.relation, rhs.to_string()])?;
    }

    writer.flush()?;
    Ok(())
}
