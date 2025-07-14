//! # Ligen
//! Ligen (Language Interface Generator) is an extensible macro-based multi-language binding
//! generator.
//!
//! We officially support binding generators for several languages, including C, Python, Rust, and more.
//!
//! ### Requirements
//!
//! - Install `cargo-ligen` via `cargo install cargo-ligen` (if using the CLI).
//! - For LLM features, set `OPENAI_API_KEY` in your environment.
//!
//! ### How to Use
//!
//! Add to your `Cargo.toml` (for build.rs integration):
//! ```toml
//! [dev-dependencies]
//! ligen = "0.1"
//! ligen-c = "0.1"  # Example for C bindings
//! ```
//!
//! In `build.rs`:
//! ```rust,ignore
//! use ligen::prelude::*;
//! use ligen_c::Generator as CGenerator;
//!
//! fn main() {
//!     if let Ok(library) = Library::read() {
//!         CGenerator::default().generate(&library).expect("Failed to generate C bindings");
//!     }
//! }
//! ```
//!
//! Generate bindings: `cargo ligen` (passes args to `cargo build`).
//!
//! For other languages, see the ecosystem crates (e.g., `ligen-python-parser`).
//!
//! ### Getting Started Resources
//! - [Supported Languages](https://github.com/sensorial-systems/ligen/search?q=orgen%3Asensorial-systems+ligen)
//! - [Example: C Bindings](https://github.com/sensorial-systems/ligen-c/tree/main/examples/counter/README.md)
//!
//! #![warn(missing_copy_implementations)]
//! #![warn(missing_debug_implementations)]
//! #![warn(missing_docs)]
//! #![warn(trivial_casts)]
//! #![warn(trivial_numeric_casts)]
//! #![warn(unsafe_code)]
//! #![warn(unused_import_braces)]
//! #![warn(unused_qualifications)]

pub use ligen_macro::*;

pub mod prelude;
pub use ligen_ir as ir;
pub use ligen_utils as utils;
pub use ligen_traits as traits;
pub use ligen_transformer as transformer;
pub use ligen_common as common;
pub use ligen_transformer::generator as generator;
pub use ligen_transformer::parser as parser;
