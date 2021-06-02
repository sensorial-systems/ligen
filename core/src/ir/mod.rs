//! Ligen intermediate representation.

pub use attribute::*;
pub use constant::*;
pub use function::*;
pub use identifier::*;
pub use implementation::*;
pub use literal::*;
pub use parameter::*;
pub use types::*;

mod attribute;
mod constant;
mod function;
mod identifier;
mod implementation;
mod literal;
mod parameter;
mod types;

pub mod processing;

