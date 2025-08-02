use std::path::PathBuf;
use std::process::Command;

fn main() {
    let project_dir = PathBuf::from("libtexprintf");
    let utftex_binary = project_dir.join("src").join("utftex");

    // We haven't already built the binary
    if !utftex_binary.exists() {
        assert!(Command::new("autoreconf")
            .arg("-i")
            .current_dir(&project_dir)
            .status()
            .expect("failed to run autoreconf")
            .success());

        assert!(Command::new("./configure")
            .current_dir("libtexprintf")
            .arg("--disable-shared")
            .arg("--enable-static")
            .status()
            .expect("failed to run configure")
            .success());

        assert!(Command::new("make")
            .current_dir("libtexprintf")
            .status()
            .expect("failed to run make")
            .success());
    }

    // Links the library for FFI
    println!("cargo:rustc-link-search=native=libtexprintf/src/.libs");
    println!("cargo:rustc-link-lib=static=texprintf");
}
