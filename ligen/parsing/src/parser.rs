use std::pin::Pin;
use ligen_ir::{Identifier, Path};
use ligen_common::Result;
use crate::{GetPathTree, PathTree};

pub struct Parser<'a> {
    pub path_tree: Pin<Box<PathTree<'a>>>
}

pub trait ParseFrom<T> {
    fn parse_from(context: &Context<'_>, from: T) -> Result<Self> where Self: Sized;
}

pub trait Parse<'a, T: GetPathTree<'a>> {
    fn parse(data: T) -> Result<Self> where Self: Sized;
}

// impl<'a, T: GetPathTree<'a>, U: ParseFrom<'a, T>> Parse<'a, T> for U {
//     fn parse(data: T) -> Result<Self> where Self: Sized {
//         let path_tree = data.get_path_tree();
//         let context = Context::from(&path_tree);
//         Self::parse_from(&context, data)
//     }
// }

impl<'a> Parser<'a> {
    pub fn root_context(&'a self) -> Context<'a> {
        (&self.path_tree).into()
    }
}

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
    use crate::{Parser, PathTree};

    fn parser<'a>() -> Parser<'a> {
        let path_tree = PathTree::new("root");
        let branch = PathTree::new("branch");
        let leaf = PathTree::new("leaf");
        branch.add_child(leaf);
        path_tree.add_child(branch);
        Parser { path_tree }
    }

    #[test]
    fn test() {
        let parser = parser();

        let context = parser.root_context();
        let new_context = context.switch_to("branch");

        assert_eq!(context.path, "root".into());
        assert_eq!(new_context.path, "root::branch".into());
    }
}