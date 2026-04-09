pub mod function;
pub mod prelude;
pub mod type_definition;

use crate::prelude::*;
use std::path::PathBuf;

#[derive(Default)]
pub struct RustClientGenerator {}

impl FileGenerator<&Library> for RustClientGenerator {
    fn base_path(&self) -> PathBuf {
        PathBuf::from("rust-client")
    }

    fn generate_files(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        let folder = PathBuf::from(&library.identifier.to_kebab_case().to_string());

        // Cargo.toml
        let cargo_toml = file_set.entry(folder.join("Cargo.toml"));
        cargo_toml.write(format!(
            r#"[package]
name = "{}"
version = "{}"
edition = "2021"

[dependencies]
reqwest = {{ version = "0.11", features = ["json"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
tokio = {{ version = "1", features = ["full"] }}
"#,
            library.identifier.to_kebab_case(),
            library.metadata.version
        ));

        // src/lib.rs
        let lib_rs = file_set.entry(folder.join("src").join("lib.rs"));
        lib_rs.write(self.generate_lib(library)?);

        Ok(())
    }
}

impl RustClientGenerator {
    fn generate_lib(&self, library: &Library) -> Result<String> {
        let mut sections = Vec::new();
        sections.push("use serde::{Serialize, Deserialize};".to_string());

        // Generate structs/enums from schemas
        for type_def in &library.root_module.types {
            let type_definition_generator =
                crate::type_definition::RustTypeDefinitionGenerator::default();
            sections.push(type_definition_generator.generate(type_def, &Config::default())?);
        }

        // Generate Client struct
        sections.push(format!(
            r#"
pub struct Client {{
    client: reqwest::Client,
    base_url: String,
}}

impl Client {{
    pub fn new(base_url: impl Into<String>) -> Self {{
        Self {{
            client: reqwest::Client::new(),
            base_url: base_url.into(),
        }}
    }}
"#
        ));

        // Generate methods
        let function_generator = crate::function::RustFunctionGenerator::default();
        for function in &library.root_module.functions {
            let method = function_generator.generate(function, &Config::default())?;
            sections.push(method);
        }

        sections.push("}".to_string());

        Ok(sections.join("\n\n"))
    }
}
