//! Ligen intermediate representation.

pub use attribute::*;
pub use procedural_macro_attributes::*;
pub use constant::*;
pub use function::*;
pub use identifier::*;
pub use implementation::*;
pub use literal::*;
pub use parameter::*;
pub use types::*;
pub use path::*;

mod procedural_macro_attributes;
mod attribute;
mod constant;
mod function;
mod identifier;
mod implementation;
mod literal;
mod parameter;
mod types;
mod path;

pub mod processing;

