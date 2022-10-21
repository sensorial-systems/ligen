use super::{Visitor, ImplementationVisitor, ModuleVisitor};
use ligen_ir::{Function, Path};

// ### Create MethodVisitor and FunctionVisitor.
// ### Dev Strategy: Replace "project" project with something else and start with a simple function to test custom marshallers for String and other external objects such as rust_decimal::Decimal.

/// Function visitor.
pub type FunctionVisitor = Visitor<FunctionParent, Function>;

/// Function parent.
#[derive(Debug, Clone)]
pub enum FunctionParent {
    /// Function is associated with a type.
    Implementation(ImplementationVisitor),
    /// Function is located in a module.
    Module(ModuleVisitor)
}

impl From<&Visitor<ImplementationVisitor, Function>> for FunctionVisitor {
    fn from(visitor: &Visitor<ImplementationVisitor, Function>) -> Self {
        let parent = FunctionParent::Implementation(visitor.parent.clone());
        let current = visitor.current.clone();
        Self { parent, current }
    }
}

impl From<&Visitor<ModuleVisitor, Function>> for FunctionVisitor {
    fn from(visitor: &Visitor<ModuleVisitor, Function>) -> Self {
        let parent = FunctionParent::Module(visitor.parent.clone().into());
        let current = visitor.current.clone();
        Self { parent, current }
    }
}

// TODO: Replace is_method with function_type.
// /// Function type.
// pub enum FunctionType {
//     Function,
//     Method
// }

impl FunctionVisitor {
    /// Check if the function is a method.
    // TODO: Use these rules https://doc.rust-lang.org/reference/items/associated-items.html#methods
    pub fn is_method(&self) -> bool {
        match &self.parent {
            FunctionParent::Module(_) => false,
            FunctionParent::Implementation(_) => self.current.method.is_some()
        }
    }

    /// Returns the module path.
    pub fn path(&self) -> Path {
        match &self.parent {
            FunctionParent::Module(module) => module.path.clone(),
            FunctionParent::Implementation(implementation) => implementation.path()
        }
    }

    /// Get the parent module.
    pub fn parent_module(&self) -> &ModuleVisitor {
        match &self.parent {
            FunctionParent::Module(module) => module,
            FunctionParent::Implementation(implementation) => implementation.parent_module()
        }
    }
}
