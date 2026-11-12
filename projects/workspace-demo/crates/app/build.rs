use std::process::Command;

fn main() {
    // فقط اگر build.rs یا این متغیر عوض شد دوباره اجرا شود
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=APP_BUILD_TAG");

    let tag = std::env::var("APP_BUILD_TAG").unwrap_or_else(|_| "dev".to_string());
    let rustc = Command::new(std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into()))
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();

    println!("cargo::rustc-env=BUILD_TAG={tag}");
    println!("cargo::rustc-env=BUILD_RUSTC={}", rustc.trim());
}
