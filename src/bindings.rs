use std::os::raw::c_char;

extern "C" {

    // char * stexprintf(const char *format, ...);
    // Returns a string of the outputted latex
    pub fn stexprintf(input: *const c_char) -> *mut c_char;

    // void texfree(void *ptr);
    // Frees the memory allocated for the latex
    pub fn texfree(ptr: *mut c_char);
}
