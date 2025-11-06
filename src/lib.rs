#![no_std]
//! Zero-dependency, `no_std` + `alloc` lookup between MIME types and file
//! extensions.
//!
//! The database is embedded once at compile time and lazily parsed on first
//! use. All returned strings are `'static` and live for the entire program
//! duration.
mod cache;
use ahash::AHasher;
use core::hash::BuildHasherDefault;
use hashbrown::HashMap;
use once_cell::sync::Lazy;

/// Return the preferred file extension (without leading dot) for a MIME type.
///
/// `None` is returned when
/// - the MIME type is unknown,
/// - the JSON database failed to parse, or
/// - the entry exists but contains no extensions.
///
/// # Example
/// ```
/// # use mime_to_ext::mime_to_ext;
/// assert_eq!(mime_to_ext("image/png"), Some("png"));
/// assert_eq!(mime_to_ext("foo/bar"), None);
/// ```
pub fn mime_to_ext(mime: &str) -> Option<&'static str> {
    match cache::DB.as_ref() {
        Ok(db) => db.get(mime).and_then(|v| v.first()).map(|s| s.as_str()),
        Err(_) => None,
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
// Single upfront inversion is faster than scanning the map on every call.
#[allow(clippy::type_complexity)]
pub fn ext_to_mime(ext: &str) -> Option<&'static str> {
    static INV: Lazy<Result<HashMap<&'static str, &'static str, BuildHasherDefault<AHasher>>, ()>> =
        Lazy::new(|| match cache::DB.as_ref() {
            Ok(db) => {
                let mut map = HashMap::with_hasher(BuildHasherDefault::<AHasher>::default());
                for (mime, exts) in db.iter() {
                    for e in exts {
                        map.entry(e.as_str()).or_insert(mime.as_str());
                    }
                }
                Ok(map)
            }
            Err(_) => Err(()),
        });

    match &*INV {
        Ok(m) => m.get(ext).copied(),
        Err(_) => None,
    }
}

/// Check whether the embedded JSON database was loaded successfully.
///
/// Returns `Ok(())` if the database parsed correctly, or `Err(())` if the
/// embedded JSON was malformed.
///
/// This is a cheap runtime test; the actual parse happens only once inside
/// `cache::DB`.
///
/// # Example
/// ```
/// # use mime_to_ext::db_status;
/// assert!(db_status().is_ok());
/// ```
#[allow(clippy::result_unit_err)]
pub fn db_status() -> Result<(), ()> {
    match cache::DB.as_ref() {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the `mime_to_ext` and `ext_to_mime` functions return
    /// the correct values for known MIME types and extensions.
    ///
    /// This test ensures that the JSON database is correctly loaded
    /// and that the lookup functions work as expected.
    #[test]
    fn lookup_works() {
        // JSON is present → must succeed
        assert_eq!(mime_to_ext("image/png"), Some("png"));
        assert_eq!(ext_to_mime("png"), Some("image/png"));
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
