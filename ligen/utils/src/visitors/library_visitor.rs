use super::Visitor;
use ligen_idl::Library;
use crate::visitors::ModuleVisitor;

/// Library visitor.
pub type LibraryVisitor = Visitor<(), Library>;

impl From<Library> for LibraryVisitor {
    fn from(library: Library) -> Self {
        Self::new((), library)
    }
}

impl LibraryVisitor {
    pub fn root_module_visitor(&self) -> ModuleVisitor {
        (&self.child(self.current.root_module.clone())).into()
    }
}
