use crate::generator::{Visitor, ImplementationVisitor};
use crate::ir::{Function, Type};

/// Function visitor.
pub type FunctionVisitor = Visitor<ImplementationVisitor, Function>;

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
