use std::pin::Pin;

mod path_tree;

pub use path_tree::*;

pub struct Tree<'a, T: 'a> {
    pub parent: Option<&'a Tree<'a, T>>,
    pub data: T,
    pub children: Vec<Pin<Box<Tree<'a, T>>>>
}

impl<'a, T: 'a> Tree<'a, T> {
    pub fn new<Data: Into<T>>(data: Data) -> Pin<Box<Self>> {
        let parent = None;
        let data = data.into();
        let children = Default::default();
        Box::pin(Self { parent, data, children })
    }

    pub fn add_child(&'a self, mut tree: Pin<Box<Self>>) {
        unsafe {
            let mut_tree: Pin<&mut Self> = Pin::as_mut(&mut tree);
            Pin::get_unchecked_mut(mut_tree).parent = Some(self);
            let const_ptr = self as *const Self;
            let mut_ptr = const_ptr as *mut Self;
            let mut_self = &mut *mut_ptr;
            mut_self.children.push(tree);
        }
    }

    pub fn get_root(&'a self) -> &'a Tree<'a, T> {
        self
            .parent
            .map(|parent| parent.get_root())
            .unwrap_or(self)
    }
}

#[cfg(test)]
pub mod tests {
    use std::pin::Pin;
    use ligen_ir::Path;
    use crate::Tree;

    fn mock<'a>() -> Pin<Box<Tree<'a, Path>>> {
        let tree = Tree::new("root");
        let branch = Tree::new("root::branch");
        branch.add_child(Tree::new("root::branch::leaf"));
        tree.add_child(branch);
        tree
    }

    fn mock_inconsistent<'a>() -> Pin<Box<Tree<'a, Path>>> {
        let tree = Tree::new("root");
        tree.add_child(Tree::new("branch"));
        tree
    }

    #[test]
    fn get_root() {
        let root = mock();
        let branch = &root.children[0];
        let leaf = &branch.children[0];
        assert_eq!(root.get_root().data, "root".into());
        assert_eq!(branch.get_root().data, "root".into());
        assert_eq!(leaf.get_root().data, "root".into());
    }

    #[test]
    fn absolute_paths() {
        assert!(mock().has_consistent_absolute_paths());
        let mut mock_inconsistent = mock_inconsistent();
        assert!(!mock_inconsistent.has_consistent_absolute_paths());
        mock_inconsistent.transform_into_absolute_paths();
        assert!(mock_inconsistent.has_consistent_absolute_paths());
    }

    #[test]
    fn find() {
        let tree = mock();
        assert_eq!(tree.find("root").map(|tree| &tree.data), Some(&"root".into()));
        assert_eq!(tree.find("root::branch").map(|tree| &tree.data), Some(&"root::branch".into()));
        assert_eq!(tree.find("root::branch::leaf").map(|tree| &tree.data), Some(&"root::branch::leaf".into()));
        assert!(tree.find("root::non_existent").is_none());
    }

    #[test]
    fn find_from_relative_path() {
        let tree = mock();
        let branch = tree.find("root::branch").expect("root::branch should be present.");
        assert_eq!(branch.find_from_relative_path("").unwrap().data, "root::branch".into());
        assert_eq!(branch.find_from_relative_path("self").unwrap().data, "root::branch".into());
        assert_eq!(branch.find_from_relative_path("super").unwrap().data, "root".into());
        assert_eq!(branch.find_from_relative_path("root").unwrap().data, "root".into());
        assert_eq!(branch.find_from_relative_path("leaf").unwrap().data, "root::branch::leaf".into());
    }
}
