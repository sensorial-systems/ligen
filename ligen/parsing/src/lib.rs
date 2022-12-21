use std::pin::Pin;
use ligen_ir::Path;

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

pub type PathTree<'a> = Tree<'a, Path>;

impl<'a> PathTree<'a> {
    pub fn has_consistent_absolute_paths(&self) -> bool {
        let is_consistent_with_parent = self
            .parent
            .map(|parent| parent.data == self.data.clone().without_last())
            .unwrap_or(true);
        let has_consistent_children = self
            .children
            .iter()
            .all(|child| child.has_consistent_absolute_paths());
        is_consistent_with_parent && has_consistent_children
    }

    pub fn find<P: Into<Path>>(&'a self, path: P) -> Option<&'a PathTree<'a>> {
        let path = path.into();
        let is_equal = self.data == path;
        if is_equal {
            Some(self)
        } else {
            self
                .children
                .iter()
                .filter_map(|child| child.find(path.clone()))
                .next()
        }
    }

    pub fn find_from_relative_path<P: Into<Path>>(&'a self, path: P) -> Option<&'a Self> {
        let mut path = path.into();
        let identifier = path.pop_front();
        identifier
            .map(|identifier| {
                match identifier.name.as_str() {
                    "self" => Some(self),
                    "super" => self
                        .parent
                        .and_then(|parent| parent.find_from_relative_path(path)),
                    identifier => {
                        let root = self.get_root();
                        if root.data.last().name == identifier {
                            root.find_from_relative_path(path)
                        } else {
                            self
                                .children
                                .iter()
                                .find(|child| child.data.last().name == identifier)
                                .and_then(|child| child.find_from_relative_path(path))
                        }
                    }
                }
            })
            .unwrap_or(Some(self))
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
        assert!(!mock_inconsistent().has_consistent_absolute_paths());
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
