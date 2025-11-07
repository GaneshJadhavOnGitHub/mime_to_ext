#![no_std]
//! no_std MIME ↔ extension lookup from embedded JSON, zero OS dependencies.
//!
//! The database is embedded once at compile time and lazily parsed on first
//! use. All returned strings are `'static` and live for the entire program
//! duration.
mod cache;
use ahash::AHasher;
use core::hash::BuildHasherDefault;
use hashbrown::HashMap;
use once_cell::sync::Lazy;

/// Return the file extensions (without leading dot) for a MIME type.
///
/// Returns `None` if
/// - the MIME type is unknown,
/// - the embedded JSON database failed to parse (i.e. the crate was compiled
///   with broken data), or
/// - the entry exists but contains no extensions.
///
/// # Example
/// ```
/// # use mime_to_ext::mime_to_ext;
/// assert_eq!(mime_to_ext("image/png"), Some(&["png"][..]));
/// assert_eq!(mime_to_ext("foo/bar"), None);
/// assert_eq!(mime_to_ext("audio/mpeg"), Some(&["mp3", "mp1", "mp2"][..]));
/// ``````
pub fn mime_to_ext(mime: &str) -> Option<&'static [&'static str]> {
    match cache::DB.as_ref() {
        Some(db) => db.get(mime).map(|v| v.as_slice()),
        None => None,
    }
}
/// Return the canonical MIME type for a file extension.
///
/// `None` is returned when
/// - the extension is unknown, or
/// - the JSON database failed to parse.
///
/// # Example
/// ```
/// # use mime_to_ext::ext_to_mime;
/// assert_eq!(ext_to_mime("png"), Some("image/png"));
/// assert_eq!(ext_to_mime("QQQ"), None);
/// ```
/// Inverted map built once at first call; speed > allocations.
#[allow(clippy::type_complexity)]
pub fn ext_to_mime(ext: &str) -> Option<&'static str> {
    static INV: Lazy<Option<HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>>>> =
        Lazy::new(|| match cache::DB.as_ref() {
            Some(db) => {
                let mut map = HashMap::with_hasher(BuildHasherDefault::<AHasher>::default());
                for (&mime, exts) in db.iter() {
                    for &e in exts {
                        map.entry(e).or_insert(mime);
                    }
                }
                Some(map)
            }
            None => None,
        });

    match INV.as_ref() {
        Some(map) => map.get(ext).copied(),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the embedded JSON database loads successfully.
    /// If the JSON is malformed this test fails, preventing `cargo test` from passing.
    #[test]
    fn db_loads_successfully() {
        assert!(
            cache::DB.is_some(),
            "embedded JSON database failed to parse"
        );
    }

    /// Unit-test: MIME type that maps to several extensions.
    ///
    /// `audio/mpeg` is registered for more than one extension; the function
    /// must return the **complete** slice in the same order stored in the JSON
    /// database.
    #[test]
    fn mime_with_multiple_extensions() {
        assert_eq!(mime_to_ext("audio/mpeg"), Some(&["mp3", "mp1", "mp2"][..]));
    }

    /// Tests that the `mime_to_ext` and `ext_to_mime` functions return
    /// the correct values for known MIME types and extensions.
    ///
    /// This test ensures that the JSON database is correctly loaded
    /// and that the lookup functions work as expected.
    #[test]
    fn lookup_works() {
        // JSON is present → must succeed
        assert_eq!(mime_to_ext("image/png"), Some(&["png"][..]));
        assert_eq!(ext_to_mime("png"), Some("image/png"));
        assert_eq!(ext_to_mime("mp1"), Some("audio/mpeg"));
    }

    /// Tests that the `mime_to_ext` and `ext_to_mime` functions return
    /// `None` for unknown MIME types and extensions.
    ///
    /// This test ensures that the lookup functions handle unknown values correctly.
    #[test]
    fn unknown_gives_none() {
        assert_eq!(mime_to_ext("foo/bar"), None);
        assert_eq!(ext_to_mime("qqq"), None);
    }
}
