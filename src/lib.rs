mod bindings; // Bindings for libtexprintf

use std::ffi::{CStr, CString};

use steel::steel_vm::ffi::{FFIModule, RegisterFFIFn};

steel::declare_module!(build_module);

fn latex_module() -> FFIModule {
    let mut module = FFIModule::new("steel/HeTeX");

    module.register_fn("latex-parse", latex_parse);

    module
}

fn latex_parse(latex_string: String) -> String {
    // Calling C FFI so unsafe is required
    let output = unsafe {
        // Get a pointer to the LaTeX output
        let ptr = bindings::stexprintf(
            CString::new(latex_string)
                .unwrap_or(CString::new("Failed to parse input!").unwrap_or_default())
                .into_raw(),
        );

        // This shouldn't happen, but as a safeguard
        if ptr.is_null() {
            return "Failed to render!".to_string();
        }

        let rust_output = CStr::from_ptr(ptr).to_string_lossy().to_string();

        bindings::texfree(ptr);

        rust_output
    };

    let mut formatted_latex_string: String = output.to_string() + " "; // the '+ " "' is to attempt to center the boxes

    // Since the command is echoed, potential attack vector
    // echo '' && notify-send example && echo ''
    // Therefore we remove all "'" (libtexprintf formats these as "`" for primes)
    formatted_latex_string = formatted_latex_string.replace("'", "");

    // The echo popup renders with a space at the start of expressions, this is a hack that formats it the way it should appear.
    // I want to make a custom popup to avoid this but as of writing this it is very verbose and poorly documented.
    formatted_latex_string.replace("\n", " \n ")
}

pub fn build_module() -> FFIModule {
    latex_module()
}
