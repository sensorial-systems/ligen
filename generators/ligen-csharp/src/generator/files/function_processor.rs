use super::*;
use handlebars::Handlebars;
use serde_json::{json, Value};
use ligen::ir::Module;

/// Parameter processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ParameterProcessor;

impl FunctionProcessor {
    pub fn generate_arguments(function: &FunctionVisitor) -> Vec<String> {
        function
            .current
            .inputs
            .iter()
            .map(|parameter| parameter.identifier.name.clone())
            .collect()
    }

    pub fn generate_parameters(kind: &str, function: &FunctionVisitor) -> Vec<String> {
        function
            .current
            .inputs
            .iter()
            .map(|parameter| {
                Self::generate_parameter(kind, &function.child(parameter.clone()))
            })
            .collect()
    }

    pub fn generate_type(module: &Module, kind: &str, type_: &ligen::ir::Type) -> String {
        let generics = if let ligen::ir::Type::Compound(_, generics) = &type_ {
            if generics.types.is_empty() {
                Default::default()
            } else {
                let types = generics
                    .types
                    .iter()
                    .map(|type_| Self::generate_type(module, kind, type_))
                    .collect::<Vec<_>>();
                format!("<{}>", types.join(", "))
            }
        } else {
            Default::default()
        };
        let type_ = Type::from(type_.clone()).to_string();
        let type_ = module
            .get_literal_from_path(format!("ligen::csharp::{}::{}::name", kind, type_))
            .map(|type_| type_.to_string())
            .unwrap_or(type_);
        format!("{}{}", type_, generics)
    }

    pub fn generate_parameter(kind: &str, parameter: &ParameterVisitor) -> String {
        let renderer = Handlebars::new();
        let root_module = &parameter.parent.parent_module().parent_project().root_module;
        let marshalling =root_module
            .get_literal_from_path(format!("ligen::csharp::marshal::{}::MarshalAs", parameter.current.type_.path().last()));
        let marshalling = if let Some(marshalling) = marshalling {
            Value::String(format!("[MarshalAs({})] ", marshalling))
        } else {
            Value::Null
        };
        let template = include_str!("parameter.template.cs");
        let type_ = Self::generate_type(root_module, kind, &parameter.type_);
        let values = json!({
            "marshalling": marshalling,
            "type": type_,
            "identifier": parameter.identifier.name
        });
        renderer
            .render_template(template, &values)
            .unwrap()
            .into()
    }
}

impl FileProcessorVisitor for FunctionProcessor {
    type Visitor = FunctionVisitor;

    fn process(&self, file_set: &mut FileSet, function: &Self::Visitor) {
        let renderer = Handlebars::new();
        let file = file_set.entry(&path(function.parent_module()));
        if let FunctionParent::Implementation(implementation) = &function.parent {
            let root_module = &function.parent_module().parent_project().root_module;
            let template = include_str!("function.template.cs");
            let ffi_name = format!("{}_{}", implementation.parent.definition.identifier(), function.current.identifier);
            let ffi_return_type = function
                .current
                .output
                .as_ref()
                .map(|type_| Self::generate_type(root_module, "ffi", type_))
                .unwrap_or("void".into());
            let return_type = function
                .current
                .output
                .as_ref()
                .map(|type_| Self::generate_type(root_module, "marshal", type_))
                .unwrap_or("void".into());
            let ffi_parameters = Self::generate_parameters("ffi", function);
            let mut parameters = Self::generate_parameters("marshal", function);
            let ffi_parameters = ffi_parameters.join(", ");
            let mut arguments = Self::generate_arguments(function);
            let static_ = if function.is_method() {
                parameters.remove(0);
                arguments.remove(0);
                arguments.insert(0, "this".into());
                ""
            } else {
                "static"
            };
            let parameters = parameters.join(", ");
            let arguments = arguments.join(", ");
            // FIXME: Hardcoded DllImport.
            let values = json!({
                "ffi_name": ffi_name,
                "ffi_return_type": ffi_return_type,
                "ffi_parameters": ffi_parameters,
                "return_type": return_type,
                "parameters": parameters,
                "arguments": arguments,
                "static": static_,
                "name": format!("{}", PascalCase::from(SnakeCase::try_from(function.current.identifier.name.as_str()).expect("Failed to transform function name from snake_case to PascalCase.")))
            });
            file.write(renderer.render_template(template, &values).unwrap());
        }
    }

    fn post_process(&self, _file_set: &mut FileSet, _function: &Self::Visitor) {}
}
