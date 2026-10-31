use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let script_path = Path::new(&manifest_dir).join("default_font.py");
    
    let status = Command::new("python3")
        .arg(&script_path)
        .env("OUT_DIR", &out_dir)
        .current_dir(&manifest_dir)
        .status()
        .expect("Failed to run default_font.py");
    
    if !status.success() {
        panic!("default_font.py failed with status: {}", status);
    }

    println!("cargo:rerun-if-changed=default_font.py");
}
