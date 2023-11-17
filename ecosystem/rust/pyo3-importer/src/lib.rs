pub mod prelude;
pub mod module_generator;
use ligen_utils::mapper::LanguageMap;
pub use module_generator::*;

pub mod templates;

use std::{path::PathBuf, rc::Rc, collections::HashSet};

use ligen_ir::{Library, Module, KindDefinition, Identifier, Type};
use prelude::*;

use ligen_generator::file_generator::{FileGenerator, FileSet, Template};
use is_tree::{IsTree, Visitor};

pub fn identifier_map() -> LanguageMap<Identifier> {
    let mut map = LanguageMap::new("ligen", "rust");
    map.insert(Identifier::boolean(), "bool");
    map.insert(Identifier::i8(), "i8");
    map.insert(Identifier::i16(), "i16");
    map.insert(Identifier::i32(), "i32");
    map.insert(Identifier::i64(), "i64");
    map.insert(Identifier::i128(), "i128");
    map.insert(Identifier::u8(), "u8");
    map.insert(Identifier::u16(), "u16");
    map.insert(Identifier::u32(), "u32");
    map.insert(Identifier::u64(), "u64");
    map.insert(Identifier::u128(), "u128");
    map.insert(Identifier::f32(), "f32");
    map.insert(Identifier::f64(), "f64");
    map.insert(Identifier::character(), "char");
    map.insert(Identifier::string(), "String");
    map.insert(Identifier::option(), "Option");
    map.insert(Identifier::date_time(), "pyo3::Py<pyo3::types::PyDateTime>");
    map.insert(Identifier::vector(), "Vec");
    map.insert(Identifier::opaque(), "pyo3::PyObject");
    map.insert(Identifier::dictionary(), "pyo3::Py<pyo3::types::PyDict>");
    map
}

pub fn rust_keywords() -> HashSet<String> {
    let mut set = HashSet::new();
    set.insert("type".to_string());
    set
}

#[derive(Debug, Default)]
pub struct LibraryGenerator {}

impl LibraryGenerator {
    pub fn generate_project_file(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        let file = file_set.entry(PathBuf::from(library.identifier.to_string()).join("Cargo.toml"));
        let mut template = Template::new();
        template.register_template("project", templates::CARGO)?;
        let content = template.render("project", library)?;
        file.write(content);
        Ok(())
    }

    pub fn generate_lib_file(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        let file = file_set.entry(PathBuf::from(library.identifier.to_string()).join("src").join("lib.rs"));
        let section = file.section.branch("documentation");
        section.writeln(library.metadata.description.split('\n').map(|s| format!("//! {}", s)).collect::<Vec<String>>().join("\n"));
        Ok(())
    }

    pub fn generate_readme(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        let file = file_set.entry(PathBuf::from(library.identifier.to_string()).join("README.md"));
        file.write(&library.metadata.description);
        Ok(())
    }

    fn translate_identifier(identifier: &Identifier) -> Identifier {
        let keywords = rust_keywords();
        keywords.get(&identifier.name).map(|_| format!("{}_", identifier.name)).unwrap_or_else(|| identifier.name.clone()).into()
    }

    fn translate_type(type_: &Type) -> Type {
        let map = identifier_map();
        let mut path = type_.path.clone();
        path.segments.iter_mut().for_each(|segment| {
            let identifier = map.get("ligen", &segment.identifier).unwrap_or(&segment.identifier).clone();
            segment.identifier = identifier;
            segment.generics.types.iter_mut().for_each(|type_| *type_ = Self::translate_type(type_));
        });
        path.into()
    }

    pub fn generate_module(&self, library: &Library, visitor: Rc<Visitor<'_, Module>>, file_set: &mut FileSet) -> Result<()> {
        let path = if visitor.path.segments.is_empty() {
            "lib".to_string()
        } else {
            visitor.path.segments.iter().map(|identifier| identifier.name.clone()).collect::<Vec<String>>().join("/")        };
        let file_path = PathBuf::from(library.identifier.to_string()).join("src").join(path).with_extension("rs");
        println!("Generating {}", file_path.display());
        let file = file_set.entry(file_path);

        let modules = file.branch("modules");
        for module in &visitor.value.modules {
            modules.writeln(format!("pub mod {};", module.identifier));
        }

        let types = file.branch("types");
        for type_ in &visitor.value.types {
            types.write(format!("pub struct {}", type_.identifier));
            // TODO: Write generics.
            types.writeln(" {");
            if let KindDefinition::Structure(structure) = &type_.definition {
                for field in &structure.fields {
                    let name = field.identifier.as_ref().map(|identifier| format!("{}: ", Self::translate_identifier(identifier))).unwrap_or_default();
                    let type_ = Self::translate_type(&field.type_);
                    types.writeln(format!("    pub {}{},", name, type_));
                }
            }
            types.writeln("}");
        }
        Ok(())
    }
}

impl FileGenerator for LibraryGenerator {
    type Input = Library;
    fn base_path(&self) -> PathBuf {
        PathBuf::from("rust".to_string())
    }

    fn generate_files(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        self.generate_project_file(library, file_set)?;
        self.generate_lib_file(library, file_set)?;
        library.root_module.iter().try_for_each(|module| self.generate_module(library, module, file_set))?;
        Ok(())
    }
}
