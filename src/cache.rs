//! One-time load, zero-copy lookup tables for MIME ↔ extension mappings.
//!
//! The embedded JSON file (`/data/mime_db.json`) is parsed **once** on first
//! access and stored in a `HashMap`.  
//! Everything is `no_std` + `alloc` only; no further heap allocations occur
//! after the initial parse.
//!
//! Public API is exposed through the root `lib.rs`; this module is an
//! implementation detail.
use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;
use once_cell::sync::Lazy;
extern crate alloc;

/// Internal type: maps a MIME type to its associated extensions.
type JsonDb = HashMap<String, Vec<String>>;

/// Raw JSON bytes compiled into the binary.
static JSON_SOURCE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/mime_db.json"));

/// Lazily-loaded, globally-shared database.  
/// First access parses the JSON; every later call re-uses the same `JsonDb`.
pub static DB: Lazy<Result<JsonDb, serde_json::Error>> =
    Lazy::new(|| serde_json::from_str(JSON_SOURCE));
