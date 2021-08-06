//! Ligen intermediate representation.

pub use constant::*;
pub use function::*;
pub use function::parameter::*;
pub use identifier::*;
pub use implementation::*;
pub use literal::*;
pub use path::*;
pub use macro_attributes::*;
pub use structure::*;
pub use types::*;
pub use visibility::*;

mod macro_attributes;
mod constant;
mod function;
mod visibility;
mod identifier;
mod implementation;
mod literal;
mod types;
mod path;
mod structure;

pub mod processing;

