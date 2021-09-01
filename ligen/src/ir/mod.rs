//! Ligen intermediate representation.

pub use module::*;
pub use constant::*;
pub use function::*;
pub use function::parameter::*;
pub use identifier::*;
pub use implementation::*;
pub use literal::*;
pub use path::*;
pub use macro_attributes::*;
pub use types::*;
pub use visibility::*;
pub use object::*;
pub use project::*;

mod module;
mod macro_attributes;
mod constant;
mod function;
mod visibility;
mod identifier;
mod implementation;
mod literal;
mod types;
mod path;
mod object;
mod project;

pub mod processing;

