//! Generator visitor module.

pub use enumeration_visitor::*;
pub use function_visitor::*;
pub use module_visitor::*;
pub use object_visitor::*;
pub use parameter_visitor::*;
pub use project_visitor::*;
pub use structure_visitor::*;
pub use import_visitor::*;

use crate::prelude::*;

mod project_visitor;
mod module_visitor;
mod object_visitor;
mod enumeration_visitor;
mod structure_visitor;
mod function_visitor;
mod parameter_visitor;
mod import_visitor;

/// Generic visitor type.
#[derive(Debug, Clone, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Visitor<Parent, Current> {
    /// Visitor's parent.
    pub parent: Parent,
    /// Currently visited.
    #[shrinkwrap(main_field)]
    pub current: Current
}

impl<Parent, Current> Visitor<Parent, Current> {
    /// Creates a new Visitor.
    pub fn new(parent: Parent, current: Current) -> Self {
        Self { parent, current }
    }

    /// Creates a new child.
    pub fn child<Child>(&self, current: Child) -> Visitor<Visitor<Parent, Current>, Child>
        where Parent: Clone,
              Current: Clone
    {
        let parent = self.clone();
        Visitor { parent, current }
    }
}
