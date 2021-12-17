//! Utils for processing the IR.

use crate::generator::FileSet;

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