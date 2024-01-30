//! Library representation.

pub mod metadata;
pub use metadata::*;

use crate::Identifier;
use crate::Interface;
use crate::Method;
use crate::Module;
use crate::Type;
use crate::prelude::*;

/// Library representation.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Library {
    pub identifier: Identifier,
    pub metadata: Metadata,
    pub root_module: Module,
}

impl Library {
    /// Save library to file.
    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<()> {
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Load library from file.
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
}

// FIXME: Remove this.
// impl IntoIterTypeMut<Type> for Library {
//     fn type_iterator(&mut self) -> TypeIterMut<'_, Type> {
//         self.root_module.iter_type_mut::<Type>()
//     }
// }

// impl IntoIterTypeMut<Method> for Library {
//     fn type_iterator(&mut self) -> TypeIterMut<'_, Method> {
//         self.root_module.iter_type_mut::<Method>()
//     }
// }

// impl IntoIterTypeMut<Interface> for Library {
//     fn type_iterator(&mut self) -> TypeIterMut<'_, Interface> {
//         self.root_module.iter_type_mut::<Interface>()
//     }
// }