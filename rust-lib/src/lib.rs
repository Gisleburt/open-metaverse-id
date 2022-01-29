use libc::c_char;
use std::ffi::CString;

fn create_returnable_c_string<S: Into<Vec<u8>>>(string: S) -> *mut c_char {
    let c_string = CString::new(string).unwrap();
    c_string.into_raw()
}

fn free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern "C" fn create_root_identity() -> *mut c_char {
    create_returnable_c_string("ToDo: This will be a root certificate")
}

#[no_mangle]
pub extern "C" fn free_root_identity(string: *mut c_char) {
    free_string(string)
}
