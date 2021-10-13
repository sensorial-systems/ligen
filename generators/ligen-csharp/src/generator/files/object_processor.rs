use super::*;

/// Object processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ObjectProcessor;

impl FileProcessorVisitor for ObjectProcessor {
    type Visitor = ObjectVisitor;

    fn process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let name = visitor
            .parent_module()
            .parent_project()
            .root_module
            .get_literal_from_path(format!("ligen::csharp::marshal::{}::name", visitor.current.definition.identifier()))
            .map(|literal| literal.to_string())
            .unwrap_or(visitor.current.definition.identifier().name.clone());
        let file = file_set.entry(&path(visitor.parent_module()));
        match &visitor.current.definition {
            TypeDefinition::Enumeration(_) => {
                file.writeln(format!("\tpublic enum {}", name));
            },
            TypeDefinition::Structure(_) => {
                file.writeln("\t[StructLayout(LayoutKind.Sequential, Pack = 1)]");
                file.writeln(format!("\tpublic struct {}", name));
            }
        }
        file.writeln("\t{");

    }

    fn post_process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(visitor.parent_module()));
        file.writeln("\t}");
    }
}
