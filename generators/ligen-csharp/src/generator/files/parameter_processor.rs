use super::*;

/// Function processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct FunctionProcessor;

impl FileProcessorVisitor for ParameterProcessor {
    type Visitor = ParameterVisitor;

    fn process(&self, _file_set: &mut FileSet, _parameter: &Self::Visitor) {}
    fn post_process(&self, _file_set: &mut FileSet, _parameter: &Self::Visitor) {}
}
