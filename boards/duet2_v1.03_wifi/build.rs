use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let memory = if env::var("CARGO_FEATURE_APPLICATION").is_ok() {
        include_bytes!("application.x").to_vec()
    } else if env::var("bootloader").is_ok() {
        include_bytes!("bootloader.x").to_vec()
    } else {
        include_bytes!("memory.x").to_vec()
    };
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(&memory)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
