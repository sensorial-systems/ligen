//! File generator module.

use crate::generator::{FileSet, Context, ImplementationVisitor, FunctionVisitor, ParameterVisitor, FileProcessorVisitor};
use crate::ir::ImplementationItem;

/// File generator.
pub trait FileGenerator {
    /// Generate files.
    fn generate_files(&self, context: &Context, file_set: &mut FileSet, visitor: Option<&ImplementationVisitor>);
}

/// File generator with visitors.
pub trait FileGeneratorVisitors {
    /// Implementation processor.
    type ImplementationProcessor: FileProcessorVisitor<Visitor = ImplementationVisitor>;

    /// Function processor.
    type FunctionProcessor: FileProcessorVisitor<Visitor = FunctionVisitor>;

    /// Parameter processor.
    type ParameterProcessor: FileProcessorVisitor<Visitor = ParameterVisitor>;
}

impl<T: FileGeneratorVisitors> FileGenerator for T {
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
