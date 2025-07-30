// use std::{env, fs, os::unix::fs::PermissionsExt, process::Command};

// fn main() {
//     let bin_path = env::temp_dir().join("utftex");

//     // Write utftex binary
//     fs::write(&bin_path, include_bytes!("../libtexprintf/src/utftex")).unwrap();

//     let mut perms = fs::metadata(&bin_path).unwrap().permissions();
//     perms.set_mode(0o755);
//     fs::set_permissions(&bin_path, perms).unwrap();

//     Command::new(&bin_path)
//         .arg(r#"$\frac{d(x^T x)}{dx}$"#)
//         .status()
//         .unwrap();

//     fs::remove_file(&bin_path).ok(); // Remove the temp file 
// }

use helix_latex::build_module;

fn main() {
    build_module()
        .emit_package_to_file("libhelix_latex", "hetex.scm")
        .unwrap()
}
