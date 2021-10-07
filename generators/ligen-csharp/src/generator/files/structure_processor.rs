use super::*;

/// Structure processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct StructureProcessor;

impl FileProcessorVisitor for StructureProcessor {
    type Visitor = StructureVisitor;

    fn process(&self, file_set: &mut FileSet, structure: &Self::Visitor) {
        let file = file_set.entry(&path(structure.parent_module()));

        let name = crate::ast::RENAME_MAP.get(&structure.identifier.name).unwrap_or(&structure.identifier.name);

        let fields: Vec<_> = structure
            .fields
            .iter()
            .filter(|field| field.identifier.is_some())
            .map(|field| (field.type_.clone(), field.identifier.clone().unwrap()))
            .collect();

        for (type_, identifier) in &fields {
            file.write("\t\t");
            if let Some(marshalling) = crate::ast::MAP_MARSHALLING.get(&type_.path().last().name) {
                file.write(format!("{} ", marshalling));
            }
            file.writeln(format!("public readonly {} {};", Type::from(type_.clone()), identifier));
        }
        file.writeln("");

        if !fields.is_empty() {
            let arguments = fields
                .iter()
                .map(|(type_, identifier)| {
                    let marshalling = crate::ast::MAP_MARSHALLING.get(&type_.path().last().name)
                        .map(|value| format!(" {}", value))
                        .unwrap_or("".into());
                    format!("{}{} {}", marshalling, Type::from(type_.clone()), identifier)
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

    fn post_process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}
}
