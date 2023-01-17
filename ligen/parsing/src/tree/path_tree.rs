use ligen_ir::Path;
use std::pin::Pin;
use crate::Tree;

pub trait GetPathTree<'a> {
    type Visitor;
    fn get_root_visitor(&self) -> &Self::Visitor;

    fn get_path_tree(&self) -> Pin<Box<PathTree<'a>>> {
        let mut path_tree = self.get_path_tree_with_visitor(self.get_root_visitor());
        path_tree.transform_into_absolute_paths();
        path_tree
    }

    fn get_path<'b>(&'b self, visitor: &'b Self::Visitor) -> Path;

    fn get_children<'b>(&'b self, visitor: &'b Self::Visitor) -> Vec<&'b Self::Visitor>;

    fn get_path_tree_with_visitor(&self, visitor: &Self::Visitor) -> Pin<Box<PathTree<'a>>> {
        let tree = PathTree::new(self.get_path(visitor));
        for child in self.get_children(visitor) {
            tree.add_child(self.get_path_tree_with_visitor(child))
        }
        tree
    }
}

pub type PathTree<'a> = Tree<'a, Path>;

impl<'a> PathTree<'a> {
    pub fn transform_into_absolute_paths(&mut self) {
        let identifier = self.data.clone();
        self.data = if let Some(parent) = self.parent {
            parent.data.clone().join(identifier)
        } else {
            identifier
        };
        for child in &mut self.children {
            child.transform_into_absolute_paths()
        }
    }

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
    use crate::PathTree;

    fn mock<'a>() -> Pin<Box<PathTree<'a>>> {
        let tree = PathTree::new("root");
        let branch = PathTree::new("root::branch");
        branch.add_child(PathTree::new("root::branch::leaf"));
        tree.add_child(branch);
        tree
    }

    fn mock_inconsistent<'a>() -> Pin<Box<PathTree<'a>>> {
        let tree = PathTree::new("root");
        tree.add_child(PathTree::new("branch"));
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
