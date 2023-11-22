//! Enumeration representation.

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub mod variant;
use is_tree::{IntoIterTypeMut, TypeIterMut};
pub use variant::*;

use crate::{prelude::*, Type};

/// Enumeration representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enumeration {
    /// Variants field.
    pub variants: Vec<Variant>,
}

impl IntoIterTypeMut<Type> for Enumeration {
    fn type_iterator(&mut self) -> TypeIterMut<'_, Type> {
        self.variants.iter_mut().flat_map(|m| m.type_iterator()).collect::<Vec<_>>().into()
    }
}