use super::*;

/// Function processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct FunctionProcessor;

impl FileProcessorVisitor for ParameterProcessor {
    type Visitor = ParameterVisitor;

    fn process(&self, _file_set: &mut FileSet, _parameter: &Self::Visitor) {
        // let renderer = Handlebars::new();
        // let file = file_set.entry(&path(parameter.parent.parent_module()));
        // let marshalling = parameter
        //     .parent
        //     .parent_module()
        //     .parent_project()
        //     .root_module
        //     .get_literal_from_path(format!("ligen::csharp::marshal::{}::MarshalAs", parameter.current.type_.path().last()));
        // let marshalling = if let Some(marshalling) = marshalling {
        //     Value::String(format!("[MarshalAs({})] ", marshalling))
        // } else {
        //     Value::Null
        // };
        // let template = include_str!("parameter.template.cs");
        // let values = json!({
        //     "marshalling": marshalling,
        //     "type": Type::from(parameter.current.type_.clone()).to_string(),
        //     "identifier": parameter.identifier.name
        // });
        // file.write(renderer.render_template(template, &values).unwrap());
    }

    fn post_process(&self, _file_set: &mut FileSet, _parameter: &Self::Visitor) {
        // let file = file_set.entry(&path(parameter.parent.parent_module()));
        // file.write(", ");
    }
}
