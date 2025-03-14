use std::ffi::CString;

pub fn c_ptr(s: &str) -> *const std::os::raw::c_char {
    CString::new(s).unwrap().into_raw()
}
