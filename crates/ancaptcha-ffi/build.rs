extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let package_name = env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "ancaptcha".to_string());
    let output_file = PathBuf::from(&crate_dir)
        .join("include")
        .join(format!("{package_name}.h"));

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(output_file);
}
