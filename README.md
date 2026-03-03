# mime_to_ext

![Rust 1.85+](https://img.shields.io/badge/rust-1.85+-orange.svg)
![no_std](https://img.shields.io/badge/no__std-compatible-green.svg)
[![Crates.io](https://img.shields.io/crates/v/mime_to_ext)](https://crates.io/crates/mime_to_ext)
[![GitHub](https://img.shields.io/badge/GitHub-181717?logo=github&logoColor=white)](https://github.com/GaneshJadhavOnGitHub/mime_to_ext)
[![Docs.rs](https://img.shields.io/badge/Docs.rs-000000?logo=docs.rs&logoColor=white)](https://docs.rs/mime_to_ext/latest)



```no_std```  MIME to extension (& vice versa) lookup using lazily-loaded embedded JSON with no runtime dependency on OS mime files.

MSRV: 1.85 (2024 edition)

####   Crate Overview

> **mime_to_ext** is a ```no_std``` crate that ships an embedded, lazily-loaded JSON mapping of comprehensive MIME-type ↔ extension pairs, giving you robust, cross-platform runtime lookup without touching OS mime files.
> Unlike other crates that rely on the system files or partial datasets, `mime_to_ext` offers one of the most complete MIME–extension mappings available in Rust — covering more than **1,100 MIME entries** from diverse sources.

---

####  Why `mime_to_ext`?

>Most crates that handle MIME lookups rely on the operating systems' limited datasets which can lead to missing mappings.
>
> `mime_to_ext` eliminates this by combining data from multiple independent and widely used MIME databases into a single unified dataset.
>
> This approach ensures:
>
> * **Cross-platform reliability** - No dependency on operating systems' limited datasets
> * **Broadest coverage** - Over *1,100 comprehensive MIME–extension pairs*
> * **Runtime lookups** - Without I/O overhead as the dataset is already embedded



---

####   How it works 

> The crate uses a precompiled JSON database that merges and normalizes data from several MIME registries and open datasets. The merging process removes duplicates, resolves conflicts, and standardizes both MIME types and extensions to provide clean bidirectional lookup — all accessible at runtime.

---


### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mime_to_ext = "0.1.14"
```

## Example 

```rust

use mime_to_ext::{mime_to_ext, ext_to_mime};

fn main() {
    // MIME → All extensions
    if let Some(exts) = mime_to_ext("audio/mpeg") {
        println!("Extensions for audio/mpeg: {}", exts.join(", "));
    } else {
        println!("No extensions found for audio/mpeg");
    }

    // Extension → Single canonical MIME type
    if let Some(mime) = ext_to_mime("png") {
        println!("The MIME type for .png is: {}", mime);
    } else {
        println!("No MIME type found for .png");
    }
}
```
### Test

```rust
assert_eq!(mime_to_ext("audio/mpeg"), Some(&["mp3", "mp1", "mp2"][..]));
assert_eq!(mime_to_ext("image/png"), Some(&["png"][..]));
assert_eq!(ext_to_mime("png"), Some("image/png"));
assert_eq!(ext_to_mime("mp1"), Some("audio/mpeg"));

assert_eq!(mime_to_ext("foo/bar"), None);
assert_eq!(ext_to_mime("qqq"), None);

```

## Quick Links

🔗 Source code: [GitHub](https://github.com/GaneshJadhavOnGitHub/mime_to_ext)  
📦 Rust crate: [crates.io](https://crates.io/crates/mime_to_ext)  
📚 Documentation: [Docs.rs](https://docs.rs/mime_to_ext/latest)


## License and Data Integrity

This project is dual-licensed under the **Apache-2.0** or **MIT** license.

### Data Standards & Stability

The mapping dataset in `mime_to_ext` is aggregated from multiple reputable upstream sources and processed using automated techniques to ensure the broadest possible coverage. While we strive for high precision and consistency, these mappings are provided on an **as-is** and **best-effort** basis.

As media types and file extensions evolve, the output of this crate may be updated in future releases to ensure continued alignment with industry standards. 

Users are encouraged to:
* **Perform downstream validation** if their application requires absolute data integrity or mission-critical certainty.
* **Always use the latest version** of this crate to ensure you are benefiting from the most recent data updates.

We actively welcome community contributions to help **expand and update the dataset**, ensuring this resource remains the most comprehensive tool for the Rust ecosystem.