use once_cell::sync::Lazy;
use std::collections::HashMap;

type JsonDb = HashMap<String, Vec<String>>;

static JSON_SOURCE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/mime_db.json"));

pub static DB: Lazy<Result<JsonDb, serde_json::Error>> =
    Lazy::new(|| serde_json::from_str(JSON_SOURCE));
