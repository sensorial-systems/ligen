use crate::ir::{Attributes, Attribute, Path};
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use quote::quote;
use std::convert::TryFrom;

/// `ligen_dependencies` macro function called by `ligen_dependencies!()`
pub fn ligen_dependencies(attributes: TokenStream) -> TokenStream {
    let attributes = Attributes::try_from(attributes).expect("Failed to parse Attributes.");
    let mut dependencies = TokenStream::new();
    let mut ligen_attributes = TokenStream::new();

    for attribute in &attributes.attributes {
        if let Attribute::Group(identifier, attribute) = attribute {
            let path = format!("ligen_{identifier}::ligen_{identifier}", identifier = identifier.name);
            let path =  Path::from(path);
            dependencies.append_all(quote! {
                use #path;
            });

            ligen_attributes.append_all(quote! {
                #identifier(#attribute)
            })
        }
    }

    let ligen_attributes = quote! {
        #[ligen(#ligen_attributes)]
    };

    quote! {
        pub mod ffi {
            use std::ffi::CStr;
            use std::os::raw::c_char;
            use ligen::ligen;
            #dependencies

            pub struct CChar(*const c_char);

            impl From<CChar> for String {
                fn from(cchar: CChar) -> Self {
                    unsafe { CStr::from_ptr(cchar.0).to_str().unwrap().to_string() }
                }
            }

            pub struct RString(std::ffi::CString);

            #ligen_attributes
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
                        )
                            .expect("Failed to create RString.");
                        Self(string)
                    }
                }
            }
        }
    }
}
