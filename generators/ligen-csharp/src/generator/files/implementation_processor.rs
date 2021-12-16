use super::*;

/// Implementation processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ImplementationProcessor;

impl FileProcessorVisitor for ImplementationProcessor {
    type Visitor = ImplementationVisitor;

    fn process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}

    fn post_process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}
}
