//! Enumeration representation.

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub mod variant;
use is_tree::{IntoIterTypeMut, TypeIteratorMut};
pub use variant::*;

use crate::{prelude::*, Type};

/// Enumeration representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enumeration {
    /// Variants field.
    pub variants: Vec<Variant>,
}

impl IntoIterTypeMut<Type> for Enumeration {
    fn into_type_iterator<'a>(&'a mut self) -> TypeIteratorMut<'a, Type> {
        self.variants.iter_mut().flat_map(|m| m.into_type_iterator()).collect::<Vec<_>>().into()
    }
}