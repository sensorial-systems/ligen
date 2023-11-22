use ligen_generator::file_generator::FileSection;
use ligen_ir::TypeDefinition;

pub use crate::prelude::*;

#[derive(Default)]
pub struct StructureGenerator {
    // identifier_generator: IdentifierGenerator,
    // type_generator: TypeGenerator,
}

impl StructureGenerator {
    pub fn generate(&self, file: &mut FileSection, type_definition: &TypeDefinition) -> Result<()> {
        file.writeln("#[derive(Debug, Clone, pyo3::FromPyObject)]");
        file.writeln("#[pyo3(transparent)]");
        file.writeln(format!("pub struct {} {{", type_definition.identifier.name));
        file.indent().writeln("pub object: pyo3::PyObject,");
        file.dedent().writeln("}\n");
        file.writeln(format!("impl pyo3::IntoPy<pyo3::PyObject> for {} {{", type_definition.identifier.name));
        file.indent().writeln("fn into_py(self, _py: pyo3::Python) -> pyo3::PyObject {");
        file.indent().writeln("self.object");
        file.dedent().writeln("}");
        file.dedent().writeln("}\n");
        // if let KindDefinition::Structure(structure) = &type_definition.definition {
        //     for field in &structure.fields {
        //         let name = field
        //             .identifier
        //             .as_ref()
        //             .map(|identifier| format!("{}", self.identifier_generator.translate(identifier)))
        //             .unwrap_or_default();
        //         let type_ = self.type_generator.translate(&field.type_);
        //         let body = file.branch("implementation").branch(&type_definition.identifier.name).branch("body");
        //         if let Visibility::Public = field.visibility {
        //             body.writeln(format!("    pub fn {name}(&self) -> {type_} {{"));
        //             body.writeln(format!("        self.{name}.clone()"));
        //             body.writeln("    }\n");
        //             body.writeln(format!("    pub fn set_{name}(&mut self, {name}: {type_}) {{"));
        //             body.writeln(format!("        self.{name} = {name};"));
        //             body.writeln("    }\n");
        //         }
        //     }
        // }
        Ok(())
    }
}
