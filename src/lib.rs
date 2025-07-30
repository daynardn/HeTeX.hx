use std::{env, fs, process::Command};

use steel::steel_vm::ffi::{FFIModule, RegisterFFIFn};

steel::declare_module!(build_module);

fn latex_module() -> FFIModule {
    let mut module = FFIModule::new("steel/HeTeX");

    module
        .register_fn("latex-parse", latex_parse);

    module
}


fn latex_parse(latex_string: String) -> String {
    let bin_path = env::temp_dir().join("utftex");

    // This writes the binary on each invokation, while small is poor practice.
    // I'd prefer to have an OnEditor start callback, and check and add it and then we know it exists
    fs::write(&bin_path, include_bytes!("../libtexprintf/src/utftex")).unwrap();
    
    // Set unix permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut perms = fs::metadata(&bin_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&bin_path, perms).unwrap();
    }

    let output = Command::new(&bin_path)
        .arg(latex_string)
        .output()
        .unwrap();

    fs::remove_file(&bin_path).ok(); // Remove the temp file 

    let formatted_latex_string: String = String::from_utf8_lossy(&output.stdout).to_string() + " "; // the '+ " "' is to attempt to center the boxes 

    // The echo popup renders with a space at the start of expressions, this is a hack that formats it the way it should appear. 
    // I want to make a custom popup to avoid this but as of writing this it is very verbose and poorly documented.
    formatted_latex_string.replace('\n', " \n ") 
}

pub fn build_module() -> FFIModule {
    latex_module()
}