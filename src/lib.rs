//! This crate provides core functionalities for ligen.

#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

pub mod ir;


use proc_macro2::TokenStream;
/// `ligen` entry-point called by `#[ligen]`.
pub fn ligen(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
