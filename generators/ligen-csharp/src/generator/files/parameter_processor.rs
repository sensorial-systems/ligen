use super::*;

/// Function processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct FunctionProcessor;

impl FileProcessorVisitor for ParameterProcessor {
    type Visitor = ParameterVisitor;

    fn process(&self, file_set: &mut FileSet, parameter: &Self::Visitor) {
        let file = file_set.entry(&path(parameter.parent.parent_module()));
        if let Some(marshalling) = crate::ast::MAP_MARSHALLING.get(&parameter.current.type_.path().last().name) {
            file.write(format!("{} ", marshalling));
        }
        file.write(format!("{} {}", Type::from(parameter.current.type_.clone()), parameter.current.identifier));
    }

    fn post_process(&self, file_set: &mut FileSet, parameter: &Self::Visitor) {
        let file = file_set.entry(&path(parameter.parent.parent_module()));
        file.write(", ");
    }
}
