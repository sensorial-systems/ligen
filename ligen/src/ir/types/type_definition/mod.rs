//! Type definitions.

mod structure;
mod enumeration;

pub use structure::*;
pub use enumeration::*;

/// All the possible ways to define a type.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum TypeDefinition {
    Structure(Structure),
    Enumeration(Enumeration)
}