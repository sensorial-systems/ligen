use is_tree::IsTree;
use schemars::JsonSchema;

use crate::{prelude::*, Library};

#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, IsTree, JsonSchema)]
#[tree(branches)]
pub struct Registry {
    pub libraries: Vec<Library>
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }
}