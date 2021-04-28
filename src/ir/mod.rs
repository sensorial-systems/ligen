//! Ligen intermediate representation.

mod argument;
mod atomic;
mod attribute;
mod function;
mod identifier;
mod impl_block;
mod impl_const;
mod literal;
mod parser;
mod types;

pub use argument::*;
pub use atomic::*;
pub use attribute::*;
pub use function::*;
pub use identifier::*;
pub use impl_block::*;
pub use impl_const::*;
pub use literal::*;
pub use parser::*;
pub use types::*;
