use super::*;
use serde_json::json;
use handlebars::Handlebars;
use crate::generator::files::type_::generate_type;

/// Structure processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct StructureProcessor;

impl FileProcessorVisitor for StructureProcessor {
    type Visitor = StructureVisitor;

    fn process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let root_module = &visitor.parent_module().parent_project().root_module;
        let identifier = &visitor.current.identifier;
        let ignore = root_module
            .get_literal_from_path(format!("ligen::{}::ignore", identifier))
            .is_some();
        if !ignore {
            let renderer = Handlebars::new();
            let file = file_set.entry(&path(visitor.parent_module()));
            let ffi_name = root_module
                .get_literal_from_path(format!("ligen::csharp::ffi::{}::name", identifier))
                .map(|literal| literal.to_string())
                .unwrap_or(visitor.current.identifier.name.clone());

            let generics = root_module
                .get_literal_from_path(format!("ligen::csharp::marshal::{}::generics", identifier))
                .map(|literal| literal.to_string())
                .unwrap_or_default();

            let methods = root_module
                .get_literal_from_path(format!("ligen::csharp::marshal::{}::methods", identifier))
                .map(|literal| literal.to_string())
                .and_then(|path| std::fs::read_to_string(&path).ok())
                .unwrap_or_default();

            let is_opaque = root_module
                .get_literal_from_path(format!("ligen::ffi::{}::opaque", identifier))
                .map(|literal| literal.to_string() == "true")
                .unwrap_or_default();

            let (ffi_fields, parameters, initialization) = if is_opaque {
                let ffi_fields = "\t\tprivate readonly IntPtr opaque;\n".to_string();
                let initialization = "\t\t\tthis.opaque = opaque;\n".to_string();
                let parameters = "IntPtr opaque".to_string();
                (ffi_fields, parameters, initialization)
            } else {
                let fields: Vec<_> = visitor
                    .current
                    .fields
                    .iter()
                    .filter(|field| field.identifier.is_some())
                    .map(|field| (field.type_.clone(), field.identifier.clone().unwrap()))
                    .collect();

                let ffi_fields = fields
                    .iter()
                    .fold(String::default(), |acc, (type_, identifier)| {
                        let marshalling = root_module
                            .get_literal_from_path(format!("ligen::csharp::marshal::{}::MarshalAs", type_.path().last()))
                            .map(|literal| format!("[MarshalAs({})] ", literal))
                            .unwrap_or_default();
                        let kind = if marshalling.is_empty() { "ffi" } else { "marshal" };
                        let type_ = generate_type(root_module, kind, &type_);
                        format!("{}\t\t{}public readonly {} {};\n", acc, marshalling, type_, identifier)
                    });
                let initialization = fields
                    .iter()
                    .fold(String::default(), |acc, (_, identifier)|
                        format!("{acc}\t\t\tthis.{identifier} = {identifier};\n", acc = acc, identifier = identifier)
                    );
                let parameters = fields
                    .iter()
                    .map(|(type_, identifier)| {
                        let kind = if root_module
                            .get_literal_from_path(format!("ligen::csharp::marshal::{}::MarshalAs", type_.path().last()))
                            .map(|literal| format!("[MarshalAs({})] ", literal))
                            .is_some() { "marshal" } else { "ffi" };
                        let type_ = generate_type(root_module, kind, &type_);
                        format!("{} {}", type_, identifier)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                (ffi_fields, parameters, initialization)
            };

            let constructor = if parameters.is_empty() {
                String::default()
            } else {
                format!("\t\tpublic {}({})\n\t\t{{\n{}\t\t}}", ffi_name, parameters, initialization)
            };

            let values = json!({
                "ffi_name": ffi_name,
                "generics": generics,
                "ffi_fields": ffi_fields,
                "constructor": constructor,
                "methods": methods
            });
            let template = include_str!("structure.template.cs");
            let content = renderer
                .render_template(template, &values)
                .unwrap();
            file.write(content);
        }
    }

    fn post_process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {

    }
}
