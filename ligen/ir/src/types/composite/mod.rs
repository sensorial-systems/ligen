use crate::{prelude::*, Path, Identifier};

pub mod generics;

pub use generics::*;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Composite {
    pub path: Path,
    pub generics: Generics
}

impl From<Identifier> for Composite {
    fn from(value: Identifier) -> Self {
        Path::from(value).into()
    }
}

impl From<Path> for Composite {
    fn from(path: Path) -> Self {
        let generics = Default::default();
        Self { path, generics }
    }
}

impl std::fmt::Display for Composite {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&format!("{}{}", self.path, self.generics))
    }
}
