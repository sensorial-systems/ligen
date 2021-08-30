//! # Ligen
//! Ligen (Language Interface Generator) is an extensible macro-based multi-language binding
//! generator.
//!
//! We officially support `ligen-c`, a binding generator for the Programming Language C.
//!
//! ### Requirements
//!
//! `cargo install cargo-ligen`
//!
//! ### How to use
//!
//! Here is an example on how to use the C generator and the CMake project generator in your crate:
//! `Cargo.toml`
//! ```toml
//! [dev-dependencies]
//! ligen       = "0.1"
//! ligen-c     = "0.1"
//! ligen-cmake = "0.1"
//! ```
//!
//! Now you can use it in your `build.rs` file as following:
//! ```rust,ignore
//! use ligen::prelude::*;
//! use ligen_c::Generator as CGenerator;
//! use ligen_cmake::Generator as CMakeGenerator;
//!
//! fn main() {
//!     if let Ok(project) = Project::read() {
//!         let c_generator = CGenerator::default();
//!         let cmake_generator = CMakeGenerator::default();
//!         cmake_generator.generate(&project).expect("Couldn't generate CMake project.");
//!         c_generator.generate(&project).expect("Couldn't generate C bindings");
//!     }
//! }
//! ```
//!
//! Then to generate the language bindings run: `cargo ligen`
//!
//! `cargo ligen` passes all its extra parameters to `cargo build`, so you can use `cargo ligen --release` to
//! generate a library with optimized production code (`--debug` is default).
//!
//! ### Getting started
//!
//! Here are a few links to get started:
//! * [List of officially supported languages](https://github.com/search?q=org%3Asensorial-systems+ligen).
//! * [Usage example](https://github.com/sensorial-systems/ligen-c/tree/main/examples/counter/README.md).

#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

extern crate proc_macro;

pub mod prelude;
pub mod ir;
pub mod utils;
pub mod generator;
pub mod error;
pub mod conventions;

pub use ligen_macro::*;