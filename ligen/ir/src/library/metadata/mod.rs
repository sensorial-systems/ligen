use std::collections::HashMap;

use crate::prelude::*;

pub mod language;
pub mod dependency;
pub mod author;
pub mod version;

pub use language::*;
pub use dependency::*;
pub use author::*;
pub use version::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Metadata {
    pub version: Version,
    pub language: Language,
    pub summary: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub authors: Vec<Author>,
    pub dependencies: Vec<Dependency>,
    pub keywords: Vec<String>,
    pub license: Option<String>,
    pub table: HashMap<String, String>,
}
