use open_meta_id::create_root_identity as create_root_identity_inner;
use libc::c_char;

use std::ffi::CString;

pub fn free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        CString::from_raw(s)
    };
}


#[no_mangle]
pub extern "C" fn create_root_identity() -> *mut c_char {
    let c_string = CString::new(create_root_identity_inner("ToDo: This will be a root certificate")).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn free_root_identity(string: *mut c_char) {
    free_string(string)
}
