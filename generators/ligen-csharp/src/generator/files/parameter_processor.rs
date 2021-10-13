use super::*;

/// Function processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct FunctionProcessor;

impl FileProcessorVisitor for ParameterProcessor {
    type Visitor = ParameterVisitor;

    fn process(&self, file_set: &mut FileSet, parameter: &Self::Visitor) {
        let file = file_set.entry(&path(parameter.parent.parent_module()));
        let marshalling = parameter
            .parent
            .parent_module()
            .parent_project()
            .root_module
            .get_literal_from_path(format!("ligen::csharp::marshal::{}::MarshalAs", parameter.current.type_.path().last()));
        if let Some(marshalling) = marshalling {
            file.write(format!("[MarshalAs({})] ", marshalling));
        }
        file.write(format!("{} {}", Type::from(parameter.current.type_.clone()), parameter.current.identifier));
    }

    fn post_process(&self, file_set: &mut FileSet, parameter: &Self::Visitor) {
        let file = file_set.entry(&path(parameter.parent.parent_module()));
        file.write(", ");
    }
}
