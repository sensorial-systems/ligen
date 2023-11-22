pub mod interface;
pub use interface::*;

use std::{rc::Rc, path::PathBuf};

use is_tree::{Visitor, IsTree};
use ligen_generator::file_generator::FileSet;
use ligen_ir::{Library, Module, Visibility, Identifier, Path};

#[derive(Default)]
pub struct ModuleGenerator {
    pub interface_generator: InterfaceGenerator
}

impl ModuleGenerator {
    pub fn generate_module(&self, library: &Library, visitor: Rc<Visitor<'_, Module>>, file_set: &mut FileSet) -> Result<()> {
        let path = if visitor.path.segments.is_empty() {
            "lib".to_string()
        } else {
            visitor.path.segments.iter().map(|identifier| identifier.name.clone()).collect::<Vec<String>>().join("/")        };
        let file_path = PathBuf::from(library.identifier.to_string()).join("src").join(path).with_extension("rs");
        println!("Generating {}", file_path.display());
        let file = file_set.entry(file_path);
        let path = Path::from(library.identifier.clone()).join(Path::from(visitor.path.clone()));
        file.writeln("lazy_static::lazy_static! {");
        file.indent().writeln("static ref PYO3_MODULE: pyo3::PyObject = {");
        file.indent().writeln("pyo3::Python::with_gil(|py| {");
        file.indent().writeln("py");
        file.indent().writeln(format!(".import(\"{}\")", path.to_string_with_separator(".")));
        file.writeln(format!(".expect(\"Failed to get {}\")", path.to_string_with_separator(".")));
        file.writeln(".into()");
        file.dedent().dedent().writeln("})");
        file.dedent().writeln("};");
        file.dedent().writeln("}\n");

        let modules = file.branch("modules");
        for module in &visitor.value.modules {
            modules.writeln(format!("pub mod {};", module.identifier));
        }
        modules.writeln("");

        let imports = file.branch("imports");
        for import in &visitor.value.imports {
            if let Visibility::Public = import.visibility {
                let first = import.path.first();
                // FIXME: This is a workaround for only importing modules inside the current library.
                if first.identifier == Identifier::root()
                || first.identifier == library.identifier
                || first.identifier == Identifier::super_()
                || first.identifier == Identifier::self_()
                { // FIXME: first.identifier == library.identifier is a hack. This case should be Identifier::root().
                    let mut path = import.path.clone();
                    if first.identifier == library.identifier {
                        path = Path::from("crate").join(import.path.clone().without_first());
                    }
                    imports.write(format!("pub use {}", path));
                    if let Some(renaming) = &import.renaming {
                        imports.write(format!(" as {}", renaming));
                    }
                    imports.writeln(";");    
                }
            }
        }
        imports.writeln("");

        let functions = file.branch("functions");
        for function in &visitor.value.functions {
            self.interface_generator.function_generator.generate_function(functions, function)?;
        }

        self.interface_generator.generate(&visitor, file)?;
        Ok(())
    }
}