//! Command-line utility for MIME ↔ extension lookup.
//! Delegates to the `mime_to_ext` no-std library.
//!
//! # Usage (from source)
//! ```bash
//! cargo run --features std --example mime_to_ext -- <mime-type|extension>
//! ```
//!
//! # Examples (without installation)
//! ```bash
//! $ cargo run --features std --example mime_to_ext -- image/png
//! png
//! $ cargo run --features std --example mime_to_ext -- png
//! image/png
//! $ cargo run --features std --example mime_to_ext -- invalid
//! ?
//! $ cargo run --features std --example mime_to_ext -- audio/mpeg
//! mp3, mp1, mp2
//! $ cargo run --features std --example mime_to_ext -- mp1
//! audio/mpeg
//! ```
//!
//! # Install locally (makes `mime_to_ext` available everywhere)
//! ```bash
//! cargo install --path . --example mime_to_ext --features std
//! ```
//!
//! # Three usage examples after installation
//! ```bash
//! mime_to_ext image/png      # → png
//! mime_to_ext png            # → image/png
//! mime_to_ext invalid        # → ?
//! mime_to_ext mp1            # → audio/mpeg
//! mime_to_ext audio/mpeg     # → mp3, mp1, mp2
//! ```
//!
//! # Uninstall
//! ```bash
//! cargo uninstall mime_to_ext
//! ```
//!
//! # Exit status
//! * 0  – successful lookup  
//! * 1  – missing / invalid argument  
//! * 2  – unknown MIME or extension (prints `?`)

use mime_to_ext::{ext_to_mime, mime_to_ext};
use std::{env, process};

/// Entry point for the mime_to_ext.
fn main() {
    let arg = match env::args().nth(1) {
        Some(a) => a,
        None => {
            eprintln!("usage: mime_to_ext <mime-type|extension>");
            process::exit(1);
        }
    };

    let out = if arg.contains('/') {
        match mime_to_ext(&arg) {
            Some(exts) => exts.join(", "),
            None => "?".to_string(),
        }
    } else {
        ext_to_mime(&arg).unwrap_or("?").to_string()
    };
    println!("{}", out);
}
