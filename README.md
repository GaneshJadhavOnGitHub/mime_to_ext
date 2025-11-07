# mime_to_ext

![Rust 1.85+](https://img.shields.io/badge/rust-1.85+-orange.svg)
![no_std](https://img.shields.io/badge/no__std-compatible-green.svg)
[![GitHub](https://img.shields.io/badge/GitHub-181717?logo=github&logoColor=white)](https://github.com/GaneshJadhavOnGitHub/mime_to_ext)
[![Crates.io](https://img.shields.io/badge/Crates.io-000000?logo=rust&logoColor=white)](https://crates.io/crates/mime_to_ext)
[![Docs.rs](https://img.shields.io/badge/Docs.rs-000000?logo=docs.rs&logoColor=white)](https://docs.rs/mime_to_ext/latest)



no_std MIME ↔ extension lookup using lazily-loaded embedded JSON with no runtime dependency on OS mime files.

MSRV: 1.85 (2024 edition)

#### 🏷️ Crate Overview

> **mime_to_ext** is a no_std + alloc crate that ships an embedded, lazily-loaded JSON mapping of verified MIME-type ↔ extension pairs, giving you robust, cross-platform runtime lookup without touching OS mime files.
> Unlike other crates that rely on system files or partial datasets, `mime_to_ext` offers one of the most complete MIME–extension mappings available in Rust — covering more than **1,100 MIME entries** from diverse, reputable sources.

---

####  Why `mime_to_ext`?

>Most crates that handle MIME lookups rely on operating systems's limited datasets which can lead to missing mappings.
>
> `mime_to_ext` eliminates this by combining data from multiple independent and widely used MIME databases into a single unified dataset.
>
> This approach ensures:
>
> * **Cross-platform reliability** - No dependency on operating systems's limited datasets
> * **Broadest coverage** - Over *1,100 verified MIME–extension pairs*
> * **Runtime lookups** - Without I/O overhead as the dataset is already embedded



---

#### ⚙️ How it works 

> The crate uses a precompiled JSON database that merges and normalizes data from several trusted MIME registries and open datasets. The merging process removes duplicates, resolves conflicts, and standardizes both MIME types and extensions to provide clean bidirectional lookup — all accessible at runtime.

---


## Usage

Add `mime_to_ext` to your `Cargo.toml`:

```toml
[dependencies]
mime_to_ext = "0.1.10"
```


## Usage Example

```rust

use mime_to_ext::{mime_to_ext, ext_to_mime};

fn main() {
    // MIME → all extensions
    if let Some(exts) = mime_to_ext("audio/mpeg") {
        println!("Extensions for audio/mpeg: {}", exts.join(", "));
    } else {
        println!("No extensions found for audio/mpeg");
    }

    // extension → single canonical MIME type
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


## License

Licensed under either of **Apache-2.0** or **MIT**.