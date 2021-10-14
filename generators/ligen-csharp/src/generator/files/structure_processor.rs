use super::*;

/// Structure processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct StructureProcessor;

impl FileProcessorVisitor for StructureProcessor {
    type Visitor = StructureVisitor;

    fn process(&self, file_set: &mut FileSet, structure: &Self::Visitor) {
        let identifier = &structure.identifier;
        let ignore = structure
            .parent_module()
            .parent_project()
            .root_module
            .get_literal_from_path(format!("ligen::{}::ignore", identifier))
            .is_some();
        if !ignore {
            let file = file_set.entry(&path(structure.parent_module()));

            let root_module = &structure.parent_module().parent_project().root_module;

            let name = root_module
                .get_literal_from_path(format!("ligen::csharp::marshal::{}::name", identifier))
                .map(|literal| literal.to_string())
                .unwrap_or(structure.current.identifier.name.clone());

            let fields: Vec<_> = structure
                .fields
                .iter()
                .filter(|field| field.identifier.is_some())
                .map(|field| (field.type_.clone(), field.identifier.clone().unwrap()))
                .collect();

            for (type_, identifier) in &fields {
                file.write("\t\t");
                let marshalling = root_module
                    .get_literal_from_path(format!("ligen::csharp::marshal::{}::MarshalAs", type_.path().last()));
                if let Some(marshalling) = marshalling {
                    file.write(format!("[MarshalAs({})] ", marshalling));
                }
                let type_ = Type::from(type_.clone()).to_string();
                let type_ = root_module
                    .get_literal_from_path(format!("ligen::csharp::marshal::{}::name", type_))
                    .map(|literal| literal.to_string())
                    .unwrap_or(type_);
                file.writeln(format!("public readonly {} {};", type_, identifier));
            }
            file.writeln("");

            if !fields.is_empty() {
                let arguments = fields
                    .iter()
                    .map(|(type_, identifier)| {
                        let marshalling = root_module
                            .get_literal_from_path(format!("ligen::csharp::marshal::{}::MarshalAs", type_.path().last()));
                        let marshalling = marshalling
                            .map(|value| format!("[MarshalAs({})] ", value))
                            .unwrap_or("".into());
                        let type_ = Type::from(type_.clone()).to_string();
                        let type_ = root_module
                            .get_literal_from_path(format!("ligen::csharp::marshal::{}::name", type_))
                            .map(|literal| literal.to_string())
                            .unwrap_or(type_);
                        format!("{}{} {}", marshalling, type_, identifier)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                file.writeln(format!("\t\tpublic {}({})", name, arguments));
                file.writeln("\t\t{");

                for (_, identifier) in fields {
                    file.writeln(format!("\t\t\tthis.{identifier} = {identifier};", identifier = identifier));
                }
                file.writeln("\t\t}");
            }
        }
    }

    fn post_process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}
}
