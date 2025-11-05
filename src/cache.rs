use once_cell::sync::Lazy;
use std::collections::HashMap;

/// A type alias for the JSON database, which maps MIME types to their corresponding file extensions.
///
/// This type is used to represent the internal data structure of the MIME type to extension mapping.
type JsonDb = HashMap<String, Vec<String>>;

/// A static string containing the JSON data for the MIME type to extension mapping.
///
/// This string is included at compile time from the `data/mime_db.json` file located in the crate's root directory.
/// The path is constructed using `env!("CARGO_MANIFEST_DIR")` to ensure it is relative to the crate root.
static JSON_SOURCE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/mime_db.json"));

/// A static instance of the MIME type to extension database, initialized lazily.
///
/// This `Lazy` instance ensures that the JSON data is parsed only once, when it is first accessed.
/// The `serde_json::from_str` function is used to parse the JSON data into a `JsonDb`.
/// Any errors during parsing are stored in the `Result` type.
pub static DB: Lazy<Result<JsonDb, serde_json::Error>> =
    Lazy::new(|| serde_json::from_str(JSON_SOURCE));
