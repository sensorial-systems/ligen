//! Ligen intermediate representation.

pub use source::*;
pub use module::{Module, Import};
pub use object::Object;
pub use function::{Function, Parameter, Synchrony, Method};
pub use identifier::Identifier;
pub use literal::Literal;
pub use path::Path;
pub use macro_attributes::{Attributes, Attribute, MacroAttributes, attributes, attribute};
pub use types::*;
pub use visibility::*;
pub use project::*;
pub use mutability::*;
pub use interface::*;

pub mod prelude;

pub mod module;
pub mod macro_attributes;
pub mod object;
pub mod function;
pub mod visibility;
pub mod identifier;
pub mod literal;
pub mod types;
pub mod interface;
pub mod path;
pub mod mutability;
pub mod source;
pub mod project;
pub mod conventions;