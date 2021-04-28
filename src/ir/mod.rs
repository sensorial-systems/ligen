//! Ligen intermediate representation.

mod argument;
mod atomic;
mod attribute;
mod constant;
mod function;
mod identifier;
mod impl_block;
mod literal;
mod parser;
mod types;

pub use argument::*;
pub use atomic::*;
pub use attribute::*;
pub use constant::*;
pub use function::*;
pub use identifier::*;
pub use impl_block::*;
pub use literal::*;
pub use parser::*;
pub use types::*;
