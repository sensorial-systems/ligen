//! This crate provides core functionalities for ligen.

#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

mod prelude;
pub mod proc_macro;

pub mod ir;
pub mod utils;

pub use proc_macro::ligen;
