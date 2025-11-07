//! Build-time validation for the embedded MIME database.
//!
//! This build script is executed automatically by Cargo before the crate is
//! compiled.  It guarantees that `data/mime_db.json` is syntactically valid
//! JSON and contains the expected structure (`Map<String, Vec<String>>`).
//! If the file is malformed the build fails immediately, preventing a broken
//! database from being shipped.
fn main() {
    println!("cargo:rerun-if-changed=data/mime_db.json");
    let json = include_str!("data/mime_db.json");
    if let Err(e) = serde_json::from_str::<serde_json::Map<String, Vec<String>>>(json) {
        eprintln!("data/mime_db.json is not valid JSON: {}", e);
        std::process::exit(1);
    }
}