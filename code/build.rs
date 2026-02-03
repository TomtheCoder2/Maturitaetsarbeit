use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let metal_source = PathBuf::from("shaders/debayer.metal");
    let air_file = out_dir.join("debayer.air");
    let metallib_file = out_dir.join("debayer.metallib");

    // Detect target for -std (macOS for your setup)
    let target = env::var("TARGET").unwrap_or_else(|_| "aarch64-apple-darwin".to_string());
    let metal_std = if target.contains("apple-darwin") {
        "-std=macos-metal2.0"  // macOS Metal 2.0 (M4 compatible; change to macos-metal1.2 for older macOS)
    } else {
        "-std=metal2.0"  // Fallback for non-macOS (but unlikely)
    };

    // Step 1: Compile Metal shader to AIR
    let output = Command::new("xcrun")
        .args([
            "metal",
            "-c",  // Compile only
            metal_source.to_str().expect("Metal source path invalid"),
            "-o", air_file.to_str().expect("AIR output path invalid"),
            metal_std,  // Fixed: macOS-specific standard
            "-O",  // Optimize
            "-fobjc-arc",  // ARC for objc if needed (harmless)
        ])
        .output()
        .expect("xcrun metal not found—install Xcode/Command Line Tools: xcode-select --install");

    if !output.status.success() {
        panic!(
            "Metal compilation failed (status: {:?}):\nStdout: {}\nStderr: {}\n\nShader file: {} exists? {}\nTarget: {}\nMetal std used: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            metal_source.display(),
            metal_source.exists(),
            target,
            metal_std
        );
    }

    // Step 2: Create metallib from AIR
    let output2 = Command::new("xcrun")
        .args([
            "metallib",
            &*air_file.to_str().expect("AIR path invalid").to_string(),
            "-o", &*metallib_file.to_str().expect("metallib path invalid").to_string(),
        ])
        .output()
        .expect("xcrun metallib not found");

    if !output2.status.success() {
        panic!(
            "metallib creation failed (status: {:?}):\nStdout: {}\nStderr: {}\nAIR file exists? {}",
            output2.status.code(),
            String::from_utf8_lossy(&output2.stdout),
            String::from_utf8_lossy(&output2.stderr),
            air_file.exists()
        );
    }

    // Clean up AIR
    let _ = std::fs::remove_file(&air_file);

    // Emit path
    println!("cargo:rustc-env=DEBAYER_METAL_LIB={}", metallib_file.display());
    println!("cargo:rerun-if-changed=shaders/debayer.metal");
}