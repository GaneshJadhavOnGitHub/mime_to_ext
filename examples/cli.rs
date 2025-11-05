use mime_to_ext::{ext_to_mime, mime_to_ext};
use std::{env, process};

/// A command-line interface for looking up MIME types and file extensions.
///
/// This program takes one argument, which can be either a MIME type or a file extension.
/// It then prints the corresponding value to the standard output.
///
/// # Usage
///
/// ```
/// cli <mime-type|extension>
/// ```
///
/// # Examples
///
/// ```
/// cli image/png
/// # Output: png
///
/// cli png
/// # Output: image/png
///
/// cli invalid
/// # Output: ?
/// ```
fn main() {
    let arg = match env::args().nth(1) {
        Some(a) => a,
        None => {
            eprintln!("usage: cli <mime-type|extension>");
            process::exit(1);
        }
    };

    let out = if arg.contains('/') {
        mime_to_ext(&arg).unwrap_or("?")
    } else {
        ext_to_mime(&arg).unwrap_or("?")
    };
    println!("{}", out);
}
