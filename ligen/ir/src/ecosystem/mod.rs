use is_tree::IsTree;

use crate::{prelude::*, Library};

#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, IsTree)]
#[tree(branches)]
pub struct Ecosystem {
    pub libraries: Vec<Library>
}

impl Ecosystem {
    pub fn new() -> Self {
        Self::default()
    }
}