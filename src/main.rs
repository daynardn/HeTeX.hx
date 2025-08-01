use helix_latex::build_module;

fn main() {
    build_module()
        .emit_package_to_file("libhelix_latex", "hetex.scm")
        .unwrap()
}
