use std::path::PathBuf;
use std::process::Command;

fn main() {
    let project_dir = PathBuf::from("libtexprintf");
    let utftex_binary = project_dir.join("src").join("utftex");

    // We've already built the file. Since it's imported, it'll never be updated
    if utftex_binary.exists() {
        return ();
    }

    assert!(
        Command::new("autoreconf")
            .arg("-i")
            .current_dir(&project_dir)
            .status()
            .expect("failed to run autoreconf")
            .success()
    );

    assert!(
        Command::new("./configure")
            .current_dir("libtexprintf")
            .arg("--disable-shared")
            .arg("--enable-static")
            .status()
            .expect("failed to run configure")
            .success()
    );

    assert!(
        Command::new("make")
            .current_dir("libtexprintf")
            .status()
            .expect("failed to run make")
            .success()
    );
}
