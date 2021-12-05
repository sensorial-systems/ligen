use ligen::marshalling::MarshalFrom;
use ligen_macro::inner_ligen;
use std::ffi::{CString, CStr};

inner_ligen! {
    ffi(
        String(
            opaque = true,
            name = "FFIString"
        ),
        FFIString(opaque = true)
    ),

    csharp(
        ffi(
            String(
                name = "FFIString"
            ),
        ),
        marshal(
            FFIString(
                methods = "src/string.methods.cs",
            ),
            String(
                name = "string",
                methods = "src/string.methods.cs",
            )
        ),
    )
}

pub struct FFIString {
    string: CString
}

impl Drop for FFIString {
    fn drop(&mut self) {
        println!("Why is {} being dropped?", self.get_pointer() as u64);
    }
}

impl FFIString {
    pub fn new(pointer: *mut i8) -> Self {
        let string = unsafe {
            CStr::from_ptr(pointer).to_owned()
        };
        Self { string }
    }

    pub fn get_pointer(&self) -> *const i8 {
        self.string.as_ptr()
    }
}

impl MarshalFrom<FFIString> for String {
    fn marshal_from(value: FFIString) -> Self {
        value.string.to_string_lossy().to_string()
    }
}

impl MarshalFrom<String> for FFIString {
    fn marshal_from(value: String) -> Self {
        let error = format!("Failed to create CString from String({}).", value);
        let string = CString::new(value).expect(&error);
        Self { string }
    }
}
