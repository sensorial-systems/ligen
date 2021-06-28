//! File generator module.

use crate::generator::{FileSet, Context, ImplementationVisitor, FunctionVisitor, ParameterVisitor, VisitorProcessor};
use crate::ir::ImplementationItem;

/// File generator.
pub trait FileGenerator {
    /// Generate files.
    fn generate_files(&self, context: &Context, file_set: &mut FileSet, visitor: Option<&ImplementationVisitor>);
}

/// File generator with visitors.
pub trait VisitorFileGenerator {
    /// Implementation processor.
    type ImplementationProcessor: VisitorProcessor<Visitor = ImplementationVisitor>;

    /// Function processor.
    type FunctionProcessor: VisitorProcessor<Visitor = FunctionVisitor>;

    /// Parameter processor.
    type ParameterProcessor: VisitorProcessor<Visitor = ParameterVisitor>;
}

impl<T: VisitorFileGenerator> FileGenerator for T {
    fn generate_files(&self, context: &Context, file_set: &mut FileSet, visitor: Option<&ImplementationVisitor>) {
        if let Some(visitor) = visitor {
            let implementation_processor = T::ImplementationProcessor::default();
            let function_processor = T::FunctionProcessor::default();
            let parameter_processor = T::ParameterProcessor::default();
            implementation_processor.process(context, file_set, visitor);
            for item in &visitor.current.items {
                if let ImplementationItem::Method(function) = item {
                    let visitor = visitor.child(function.clone());
                    function_processor.process(context, file_set, &visitor);
                    for (index, parameter) in function.inputs.iter().enumerate() {
                        let visitor = visitor.child(parameter.clone());
                        parameter_processor.process(context, file_set, &visitor);
                        if index != function.inputs.len() - 1 {
                            parameter_processor.post_process(context, file_set, &visitor);
                        }
                    }
                    function_processor.post_process(context, file_set, &visitor);
                }
            }
            implementation_processor.post_process(context, file_set, visitor);
        }
    }
}