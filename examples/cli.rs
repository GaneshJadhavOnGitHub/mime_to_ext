use mime_to_ext::{ext_to_mime, mime_to_ext};
use std::{env, process};

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