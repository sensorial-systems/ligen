//! Generator visitor module.

use crate::ir::{Implementation, Function, Parameter, Type, Object, Structure};
use crate::generator::{Context, FileSet};

/// Generic visitor type.
#[derive(Debug, Clone)]
pub struct Visitor<Parent, Current> {
    /// Visitor's parent.
    pub parent: Parent,
    /// Currently visited.
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

/// Object visitor.
pub type ObjectVisitor = Visitor<(), Object>;

/// Structure visitor.
pub type StructureVisitor = Visitor<ObjectVisitor, Structure>;

/// Implementation visitor.
pub type ImplementationVisitor = Visitor<ObjectVisitor, Implementation>;

/// Function visitor.
pub type FunctionVisitor = Visitor<ImplementationVisitor, Function>;

/// Parameter visitor.
pub type ParameterVisitor = Visitor<FunctionVisitor, Parameter>;

impl FunctionVisitor {
    /// Check if the function is a method.
    // TODO: Use these rules https://doc.rust-lang.org/reference/items/associated-items.html#methods
    pub fn is_method(&self) -> bool {
        if let Some(input) = self.current.inputs.get(0) {
            input.type_.path() == self.parent.current.self_.path() || input.type_ == Type::self_type()
        } else {
            false
        }
    }
}

/// File processor visitor.
pub trait FileProcessorVisitor: Default {
    /// Visitor's type.
    type Visitor;

    /// Processor executed while visiting the current element and before visiting its children.
    fn process(&self, _context: &Context, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}

    /// Post-processor executed after visiting the current element and its children.
    /// It has a special behavior for `ParameterVisitor`: It only executes if the `parameter` isn't
    /// the last parameter, which is useful for writing separators.
    fn post_process(&self, _context: &Context, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}
}
