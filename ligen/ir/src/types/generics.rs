use crate::Type;
use crate::prelude::*;

/// Generic arguments list.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Generics {
    /// Generic types.
    pub types: Vec<Type>
}

impl std::fmt::Display for Generics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.types.is_empty() {
            f.write_str("")
        } else {
            let generics = self
                .types
                .iter()
                .map(|generic| format!("{}", generic))
                .collect::<Vec<String>>()
                .join(", ");
            f.write_str(&format!("<{}>", generics))
        }
    }
}
