pub mod function;
pub mod structure;
pub use function::*;
pub use structure::*;

use is_tree::{HasBranch, Visitor};
use ligen::idl::{Module, Visitors};

#[derive(Default)]
pub struct InterfaceGenerator {
    pub function_generator: FunctionGenerator,
    structure_generator: StructureGenerator
}

impl InterfaceGenerator {
    // pub fn generate(&self, visitor: &Rc<Visitor<'_, Module>>, file: &mut File) -> Result<()> {
    pub fn generate(&self, visitor: &Visitor<Box<Visitors>, &Module>, file: &mut File) -> Result<()> {
        // TODO: Use template here instead of creating the sections structure manually.
        for type_ in &visitor.value.types {
            let implementation = file.section.branch("implementation").branch(&type_.identifier.name);
            implementation.branch("begin").writeln(format!("impl {} {{", type_.identifier));
            implementation.branch("body");
            implementation.branch("end").writeln("}\n");
        }

        for type_ in &visitor.value.types {
            self.structure_generator.generate(file, type_)?;
        }

        let implementation = file.section.branch("implementation");

        implementation.writeln("lazy_static::lazy_static! {");
        implementation.indent().writeln(format!("static ref PYO3_{}: pyo3::PyObject = {{", visitor.value.identifier.name.to_uppercase()));
        implementation.indent().writeln("pyo3::Python::with_gil(|py| {");
        implementation.indent().writeln("PYO3_MODULE");
        implementation.indent().writeln(format!(".getattr(py, \"{}\")", visitor.value.identifier));
        implementation.writeln(format!(".expect(\"Failed to get {}\")", visitor.value.identifier));
        implementation.writeln(".into()");
        implementation.dedent().dedent().writeln("})");
        implementation.dedent().writeln("};");
        implementation.dedent().writeln("}\n");

        for interface in &visitor.value.interfaces {
            let body = implementation.branch(&interface.identifier.name).indented_branch("body");
            for method in &interface.methods {
                self.function_generator.generate_method(body, method)?;
            }
            for function in &interface.functions {
                self.function_generator.generate_function(body, function)?;
            }
        }
        Ok(())
    }
}