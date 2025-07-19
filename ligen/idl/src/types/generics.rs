use crate::Type;
use crate::prelude::*;

/// Generic arguments list.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
pub struct Generics {
    /// Generic types.
    pub types: Vec<Type>
}

impl<T: Into<Type>> From<Vec<T>> for Generics {
    fn from(value: Vec<T>) -> Self {
        let types = value
            .into_iter()
            .map(|type_| type_.into())
            .collect();
        Self { types }
    }
}

impl From<&str> for Generics {
    fn from(value: &str) -> Self {
        Type::from(value).into()
    }
}

impl From<Type> for Generics {
    fn from(value: Type) -> Self {
        let types = vec![value];
        Self { types }
    }
}

impl std::fmt::Display for Generics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.types.is_empty() {
            f.write_str("")
        } else {
            let generics = self
                .types
                .iter()
                .map(|generic| format!("{generic}"))
                .collect::<Vec<String>>()
                .join(", ");
            f.write_str(&format!("<{generics}>"))
        }
    }
}
