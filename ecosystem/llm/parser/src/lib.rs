pub mod prelude;

// IR
mod library;
mod function;
mod module;

// Generic parser
mod generic_parser;
pub mod discovery;

// Type descriptor
mod type_descriptor;

pub mod schema;

pub use library::*;
pub use generic_parser::*;
pub use type_descriptor::*;
