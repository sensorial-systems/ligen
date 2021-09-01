//! Generator visitor module.

use crate::prelude::*;
use crate::ir::{Implementation, Function, Parameter, Type, Object, Structure, Module, Project, Path, Enumeration};
use crate::generator::FileSet;
use crate::conventions::naming::SnakeCase;

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

/// Project visitor.
pub type ProjectVisitor = Visitor<(), Project>;

/// Module visitor.
pub type ModuleVisitor = Visitor<ModuleParent, Module>;

/// Object visitor.
pub type ObjectVisitor = Visitor<ModuleVisitor, Object>;

/// Enumeration visitor.
pub type EnumerationVisitor = Visitor<ObjectVisitor, Enumeration>;

/// Structure visitor.
pub type StructureVisitor = Visitor<ObjectVisitor, Structure>;

/// Implementation visitor.
pub type ImplementationVisitor = Visitor<ObjectVisitor, Implementation>;

/// Function visitor.
pub type FunctionVisitor = Visitor<ImplementationVisitor, Function>;

/// Parameter visitor.
pub type ParameterVisitor = Visitor<FunctionVisitor, Parameter>;

impl ModuleVisitor {
    /// Returns the parent project.
    pub fn parent_project(&self) -> &Project {
        match &self.parent {
            ModuleParent::Project(visitor) => &visitor.current,
            ModuleParent::Module(module) => module.parent_project()
        }
    }

    /// Returns the module path.
    pub fn path(&self) -> Path {
        let mut segments = Vec::new();
        match &self.parent {
            ModuleParent::Project(project) => segments.push(SnakeCase::from(project.current.name().clone()).into()),
            ModuleParent::Module(module) => {
                segments.append(&mut module.path().segments);
                segments.push(self.current.name.clone());
            }
        }
        segments.into()
    }
}

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

/// All the possibilities of module parents.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum ModuleParent {
    Project(ProjectVisitor),
    Module(Box<ModuleVisitor>)
}

impl From<&Visitor<ProjectVisitor, Module>> for ModuleVisitor {
    fn from(visitor: &Visitor<ProjectVisitor, Module>) -> Self {
        let parent = ModuleParent::Project(visitor.parent.clone());
        let current = visitor.current.clone();
        Self { parent, current }
    }
}

impl From<&Visitor<ModuleVisitor, Module>> for ModuleVisitor {
    fn from(visitor: &Visitor<ModuleVisitor, Module>) -> Self {
        let parent = ModuleParent::Module(visitor.parent.clone().into());
        let current = visitor.current.clone();
        Self { parent, current }
    }
}

/// File processor visitor.
pub trait FileProcessorVisitor: Default {
    /// Visitor's type.
    type Visitor;

    /// Processor executed while visiting the current element and before visiting its children.
    fn process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}

    /// Post-processor executed after visiting the current element and its children.
    /// It has a special behavior for `ParameterVisitor`: It only executes if the `parameter` isn't
    /// the last parameter, which is useful for writing separators.
    fn post_process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}
}
