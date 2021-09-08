// inner_ligen!(MarshalTo(String, *mut RString));
// inner_ligen!(MarshalFrom(String, CChar));

use std::os::raw::c_char;

pub struct RString(std::ffi::CString);

impl RString {
    pub fn new(string: *const c_char) -> Self {
        string.into()
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr()
    }
}

impl From<String> for RString {
    fn from(string: String) -> Self {
        let string = std::ffi::CString::new(string).expect("Couldn't create CString.");
        Self(string)
    }
}

impl From<RString> for String {
    fn from(string: RString) -> Self {
        string.0.to_string_lossy().to_string()
    }
}

impl From<*const c_char> for RString {
    fn from(c_char: *const c_char) -> Self {
        unsafe {
            let string = std::ffi::CString::new(
                std::ffi::CStr::from_ptr(c_char)
                    .to_string_lossy()
                    .to_string(),
            ).expect("Failed to create RString.");
            Self(string)
        }
    }
}
