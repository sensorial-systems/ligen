use std::ffi::CStr;
use std::os::raw::c_char;

pub mod rstring;

#[repr(C)]
pub struct CChar(*const c_char);

impl From<CChar> for String {
    fn from(cchar: CChar) -> Self {
        unsafe { CStr::from_ptr(cchar.0).to_str().unwrap().to_string() }
    }
}