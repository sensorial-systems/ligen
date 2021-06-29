//! FFI generator module.

use crate::prelude::*;

use crate::generator::{Context, ImplementationVisitor, FunctionVisitor, ParameterVisitor, FFIProcessorVisitor};
use crate::ir::ImplementationItem;

/// Implementation processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ImplementationProcessor;

/// Function processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct FunctionProcessor;

/// Parameter processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ParameterProcessor;

/// FFI generator.
pub trait FFIGenerator {
    /// Generate FFI.
    fn generate_ffi(&self, context: &Context, visitor: Option<&ImplementationVisitor>) -> TokenStream;
}

/// File generator with visitors.
pub trait FFIGeneratorVisitors {
    /// Implementation processor.
    type ImplementationProcessor: FFIProcessorVisitor<Visitor = ImplementationVisitor>;

    /// Function processor.
    type FunctionProcessor: FFIProcessorVisitor<Visitor = FunctionVisitor>;

    /// Parameter processor.
    type ParameterProcessor: FFIProcessorVisitor<Visitor = ParameterVisitor>;
}

impl<T: FFIGeneratorVisitors> FFIGenerator for T {
    fn generate_ffi(&self, context: &Context, visitor: Option<&ImplementationVisitor>) -> TokenStream {
        let mut token_stream = TokenStream::default();
        if let Some(visitor) = visitor {
            let implementation_processor = T::ImplementationProcessor::default();
            let function_processor = T::FunctionProcessor::default();
            let parameter_processor = T::ParameterProcessor::default();
            token_stream.append_all(implementation_processor.process(context, visitor));
            for item in &visitor.current.items {
                if let ImplementationItem::Method(function) = item {
                    let visitor = visitor.child(function.clone());
                    token_stream.append_all(function_processor.process(context, &visitor));
                    for (index, parameter) in function.inputs.iter().enumerate() {
                        let visitor = visitor.child(parameter.clone());
                        token_stream.append_all(parameter_processor.process(context, &visitor));
                        if index != function.inputs.len() - 1 {
                            token_stream.append_all(parameter_processor.post_process(context, &visitor));
                        }
                    }
                    token_stream.append_all(function_processor.post_process(context, &visitor));
                }
            }
            token_stream.append_all(implementation_processor.post_process(context, visitor));
        }
        token_stream
    }
}
