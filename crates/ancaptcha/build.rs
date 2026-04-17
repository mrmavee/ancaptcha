use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| String::from("."));
    let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| String::from("target"));

    let assets_source = PathBuf::from(&manifest_dir).join("assets/default_images");
    let assets_output = PathBuf::from(&out_dir).join("processed_assets");

    if !assets_source.exists() {
        println!(
            "cargo:warning=Assets directory not found at {}",
            assets_source.display()
        );
        generate_empty_registry(&out_dir);
        return;
    }

    fs::create_dir_all(&assets_output).unwrap_or_else(|e| {
        panic!("Failed to create output directory: {e}");
    });

    let mut processed_files = Vec::new();

    let entries = fs::read_dir(&assets_source).unwrap_or_else(|e| {
        panic!("Failed to read assets directory: {e}");
    });

    for entry in entries {
        let Ok(entry) = entry else { continue };

        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        if extension != "jpeg" && extension != "jpg" && extension != "png" && extension != "webp" {
            continue;
        }

        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        let output_path = assets_output.join(format!("{file_stem}.webp"));

        if extension == "webp" {
            let size = fs::metadata(&path).map_or(u64::MAX, |m| m.len());
            if size <= 5120 {
                fs::copy(&path, &output_path).unwrap();
                processed_files.push((file_stem.to_string(), output_path.clone()));
                continue;
            }
        }

        match process_image(&path, &output_path) {
            Ok(size) if size <= 5120 => {
                processed_files.push((file_stem.to_string(), output_path.clone()));
            }
            Ok(size) => {
                println!(
                    "cargo:warning=Image {file_stem}.webp is too large ({size} bytes) even after compression. Max allowed is 5120 bytes."
                );
            }
            Err(e) => {
                println!("cargo:warning=Failed to process image {file_stem}: {e}");
            }
        }
    }

    let count = processed_files.len();
    generate_registry(&out_dir, &processed_files);

    println!("cargo:warning=anCaptcha asset pipeline: embedded {count} optimized assets.");
    println!("cargo:rerun-if-changed={}", assets_source.display());
}

fn process_image(input: &Path, output: &Path) -> Result<u64, String> {
    let img = image::open(input).map_err(|e| format!("Failed to open image: {e}"))?;

    let mut target_size = 160u32;

    loop {
        let resized = img.resize(
            target_size,
            target_size,
            image::imageops::FilterType::Lanczos3,
        );

        let mut buffer = Vec::new();
        let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut buffer);
        resized
            .write_with_encoder(encoder)
            .map_err(|e| format!("Failed to encode WebP: {e}"))?;

        if buffer.len() <= 5120 || target_size <= 40 {
            fs::write(output, &buffer).map_err(|e| format!("Failed to write output file: {e}"))?;
            return Ok(buffer.len() as u64);
        }

        target_size = target_size.saturating_mul(9).saturating_div(10);
    }
}

fn generate_registry(out_dir: &str, files: &[(String, PathBuf)]) {
    let registry_path = PathBuf::from(out_dir).join("asset_registry.rs");

    let mut registry_file = File::create(&registry_path).unwrap_or_else(|e| {
        panic!("Failed to create registry file: {e}");
    });

    writeln!(
        registry_file,
        "pub const EMBEDDED_ASSETS: &[(&str, &[u8])] = &["
    )
    .unwrap_or_else(|e| panic!("Failed to write to registry: {e}"));

    for (name, path) in files {
        let path_str = path.display().to_string().replace('\\', "/");
        writeln!(
            registry_file,
            "    (\"{name}\", include_bytes!(r\"{path_str}\")),"
        )
        .unwrap_or_else(|e| panic!("Failed to write asset entry: {e}"));
    }

    writeln!(registry_file, "];").unwrap_or_else(|e| panic!("Failed to finalize registry: {e}"));
}

fn generate_empty_registry(out_dir: &str) {
    let registry_path = PathBuf::from(out_dir).join("asset_registry.rs");
    let mut registry_file = File::create(&registry_path).unwrap_or_else(|e| {
        panic!("Failed to create empty registry file: {e}");
    });
    writeln!(
        registry_file,
        "pub const EMBEDDED_ASSETS: &[(&str, &[u8])] = &[];"
    )
    .unwrap_or_else(|e| panic!("Failed to write empty registry: {e}"));
}
