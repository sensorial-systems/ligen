// TODO: Organize this module.

use std::pin::Pin;
use ligen_ir::{Identifier, Path};
use ligen_common::Result;
use crate::PathTree;

pub trait Parser<Input> {
    type Output;
    fn parse(&self, input: Input) -> Result<Self::Output>;
}

#[derive(Clone)]
pub struct Context<'a> {
    pub path: Path,
    pub path_tree: &'a PathTree<'a>
}

impl<'a> Context<'a> {
    pub fn switch_to<I: Into<Identifier>>(&self, identifier: I) -> Context<'_> {
        let path_tree = self.path_tree;
        let path = self.path.clone().join(identifier.into());
        Self { path, path_tree }
    }
}

impl<'a> From<&'a Pin<Box<PathTree<'a>>>> for Context<'a> {
    fn from(path_tree: &'a Pin<Box<PathTree<'a>>>) -> Self {
        let path = path_tree.data.clone();
        Self { path, path_tree }
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use crate::{Context, PathTree};

    fn path_tree<'a>() -> Pin<Box<PathTree<'a>>> {
        let path_tree = PathTree::new("root");
        let branch = PathTree::new("branch");
        let leaf = PathTree::new("leaf");
        branch.add_child(leaf);
        path_tree.add_child(branch);
        path_tree
    }

    #[test]
    fn test() {
        let path_tree = path_tree();
        let context = Context::from(&path_tree);
        let new_context = context.switch_to("branch");

        assert_eq!(context.path, "root".into());
        assert_eq!(new_context.path, "root::branch".into());
    }
}