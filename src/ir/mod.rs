//! Ligen intermediate representation.

mod argument;
mod atomic;
mod attribute;
mod function;
mod identifier;
mod literal;
mod parser;
mod types;

pub use argument::*;
pub use atomic::*;
pub use attribute::*;
pub use function::*;
pub use identifier::*;
pub use literal::*;
pub use parser::*;
pub use types::*;
