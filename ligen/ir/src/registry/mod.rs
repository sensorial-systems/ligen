use is_tree::IsTree;

use crate::{prelude::*, Library};

#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, IsTree)]
#[tree(branches)]
pub struct Registry {
    pub libraries: Vec<Library>
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }
}