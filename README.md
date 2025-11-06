# mime_to_ext

![Rust 1.85+](https://img.shields.io/badge/rust-1.85+-orange.svg)
![no_std](https://img.shields.io/badge/no__std-compatible-green.svg)


Zero-dependency crate for runtime lookup between MIME types and file extensions.

MSRV: 1.85 (2024 edition)

#### 🏷️ Crate Overview

> **mime_to_ext** is a zero-dependency `no_std` + `alloc` crate that provides a robust runtime lookup between MIME types and file extensions.
> Unlike other crates that rely on system files or partial datasets, `mime_to_ext` offers one of the most complete MIME–extension mappings available in Rust — covering more than **1,100 MIME entries** from diverse, reputable sources.

---

####  Why `mime_to_ext`?

> Most crates that handle MIME type lookups depend on limited datasets — often system-level `mime.types` files or small curated tables. This can lead to missing mappings.
>
> `mime_to_ext` eliminates this by combining data from multiple independent and widely used MIME databases into a single unified dataset.
>
> The combined data ensures:
>
> * **Cross-platform reliability** - No dependency on system `mime.types`
> * **Broadest coverage** - Over *1,100 verified MIME–extension pairs*
> * **Runtime lookups** - Without any compilation or I/O overhead
> * **Zero dependencies** - For faster build times and minimal footprint

---

#### ⚙️ How it works 

> The crate uses a precompiled JSON database that merges and normalizes data from several trusted MIME registries and open datasets. The merging process removes duplicates, resolves conflicts, and standardizes both MIME types and extensions to provide clean bidirectional lookup — all accessible at runtime.

---


## Usage

Add `mime_to_ext` to your `Cargo.toml`:

```toml
[dependencies]
mime_to_ext = "0.1.9"
```


## Usage Example

```rust

use mime_to_ext::{mime_to_ext, ext_to_mime};

fn main() {
    if let Some(ext) = mime_to_ext("image/png") {
        println!("The preferred extension for image/png is: {}", ext);
    } else {
        println!("No extension found for image/png");
    }
    if let Some(mime) = ext_to_mime("png") {
        println!("The MIME type for .png is: {}", mime);
    } else {
        println!("No MIME type found for .png");
    }
}
```
### Test

```rust

assert_eq!(mime_to_ext("image/png"), Some("png"));
assert_eq!(ext_to_mime("png"), Some("image/png"));

assert_eq!(mime_to_ext("foo/bar"), None);
assert_eq!(ext_to_mime("qqq"), None);

```

## Quick Links

🔗 Source code: [GitHub](https://github.com/GaneshJadhavOnGitHub/mime_to_ext)  
📦 Rust crate: [crates.io](https://crates.io/crates/mime_to_ext)  
📚 Documentation: [Docs.rs](https://docs.rs/mime_to_ext/latest)


## License

Licensed under either of **Apache-2.0** or **MIT** at your option.