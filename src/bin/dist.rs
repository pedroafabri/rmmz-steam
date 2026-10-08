use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("🚀 Compiling NAPI library...");

    let status = Command::new("cargo")
        .args(["build", "--release", "--lib"])
        .status()
        .expect("Failed to execute cargo build");

    if !status.success() {
        eprintln!("Error during Rust compilation");
        std::process::exit(1);
    }

    let (src_filename, platform_folder) = if cfg!(target_os = "windows") {
        ("rmmz_steam.dll", "win32-x64")
    } else if cfg!(target_os = "linux") {
        ("librmmz_steam.so", "linux-x64")
    } else if cfg!(target_os = "macos") {
        ("librmmz_steam.dylib", "darwin-x64")
    } else {
        panic!("Unsupported OS.");
    };

    let src_path = Path::new("target").join("release").join(src_filename);
    let dist_dir = Path::new("dist").join(platform_folder);
    let target_path = dist_dir.join("rmmz_steam.node");

    fs::create_dir_all(&dist_dir).expect("Error while creating dist folder");
    fs::copy(&src_path, &target_path).expect("Error while copying the binary to dist folder");

    println!("✅ Build successful!");
    println!("📍 Generated file: {}", target_path.display());
}
