use crate::prelude::*;

pub mod language;
pub mod dependency;
pub mod author;
pub mod version;

pub use language::*;
pub use dependency::*;
pub use author::*;
pub use version::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    pub version: Version,
    pub language: Language,
    pub summary: String,
    pub description: String,
    pub homepage: String,
    pub authors: Vec<Author>,
    pub dependencies: Vec<Dependency>,
    pub keywords: Vec<String>,
    pub license: String,
}
