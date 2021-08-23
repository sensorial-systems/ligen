pub struct Counter {
    count: u32
}

impl Counter {
    pub fn new(count: u32) -> Self {
        Self { count }
    }

    pub fn count(&mut self, counts: u32) {
        self.count += counts;
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }
}

pub struct Person {
    pub first_name: String,
    pub last_name: String
}

impl Person {
    pub fn new(first_name: String, last_name: String) -> Self {
        Self { first_name, last_name }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
















// pub mod ffi {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    #[repr(C)]
    pub struct CChar(*const c_char);

    impl From<CChar> for String {
        fn from(cchar: CChar) -> Self {
            unsafe { CStr::from_ptr(cchar.0).to_str().unwrap().to_string() }
        }
    }

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
// }