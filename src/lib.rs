mod cache;

/// Return the **preferred** file extension (without dot) for a given MIME type.
///
/// Returns `None` if the MIME type is unknown **or** the database failed to load.
/// or the MIME entry exists but contains no extensions.
///
/// # Example
/// ```
/// assert_eq!(mime_to_ext::mime_to_ext("image/png"), Some("png"));
/// assert_eq!(mime_to_ext::mime_to_ext("foo/bar"), None);
/// ```
pub fn mime_to_ext(mime: &str) -> Option<&'static str> {
    match cache::DB.as_ref() {
        Ok(db) => db.get(mime).and_then(|v| v.first()).map(|s| s.as_str()),
        Err(_) => None,
    }
}

/// Return the **canonical** MIME type for a file extension (lower-case, no dot).
///
/// Returns `None` if the extension is unknown **or** the database failed to load.
/// (Empty extension arrays in the JSON do not affect this function.)
/// # Example
/// ```
/// assert_eq!(mime_to_ext::ext_to_mime("png"), Some("image/png"));
/// assert_eq!(mime_to_ext::ext_to_mime("QQQ"), None);
/// ```
pub fn ext_to_mime(ext: &str) -> Option<&'static str> {
    use std::sync::OnceLock;
    static INV: OnceLock<Result<std::collections::HashMap<&'static str, &'static str>, ()>> =
        OnceLock::new();
    let inv = INV.get_or_init(|| match cache::DB.as_ref() {
        Ok(db) => {
            let mut map = std::collections::HashMap::new();
            for (mime, exts) in db.iter() {
                for e in exts {
                    map.entry(e.as_str()).or_insert(mime.as_str());
                }
            }
            Ok(map)
        }
        Err(_) => Err(()),
    });
    match inv {
        Ok(m) => m.get(ext).copied(),
        Err(_) => None,
    }
}

/// Returns `Ok(())` if the JSON database was loaded successfully.
/// Returns `Err(())` if the file was malformed.
///
/// ```
/// assert!(mime_to_ext::db_status().is_ok());
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

    #[test]
    fn lookup_works() {
        // JSON is present → must succeed
        assert_eq!(mime_to_ext("image/png"), Some("png"));
        assert_eq!(ext_to_mime("png"), Some("image/png"));
    }

    #[test]
    fn unknown_gives_none() {
        assert_eq!(mime_to_ext("foo/bar"), None);
        assert_eq!(ext_to_mime("qqq"), None);
    }
}
