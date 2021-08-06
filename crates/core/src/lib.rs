//! This crate provides core functionalities for ligen.

#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

#![cfg_attr(cargo_ligen, feature(proc_macro_span))]

extern crate proc_macro as rust_proc_macro;

pub mod prelude;
pub mod r#macro;
pub mod ir;
pub mod utils;
pub mod generator;
pub mod error;
