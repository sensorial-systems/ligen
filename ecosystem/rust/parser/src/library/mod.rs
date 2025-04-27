use crate::cargo::Cargo;
use crate::prelude::*;
use anyhow::Context;
use ligen::ir::{Identifier, Language, Library, Metadata};

use crate::module::ModuleParser;

#[derive(Default)]
pub struct RustLibraryParser {
    module_parser: ModuleParser
}

impl RustLibraryParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<&std::path::Path, Library> for RustLibraryParser {
    fn transform(&self, input: &std::path::Path, config: &Config) -> Result<Library> {
        let cargo = Cargo::new(input).context("Failed to create Cargo instance")?;
        let identifier = Identifier::from(cargo.get_name()?.clone());
        let authors = cargo.get_authors()?;
        let dependencies = cargo.get_dependencies()?;
        let keywords = cargo.get_keywords()?;
        let license = cargo.get_license()?;
        let version = cargo.get_version()?;
        let language = Language::new("Rust", cargo.get_rust_version()?);
        let summary = Default::default();
        let description = cargo.get_description()?;
        let homepage = cargo.get_homepage()?;
        let table = Default::default();
        let metadata = Metadata { authors, dependencies, keywords, license, version, language, summary, description, homepage, table };
        let root_module = self.module_parser.transform(cargo.folder.join("src").join("lib.rs").as_path(), config)?;
        let library = Library { identifier, metadata, root_module };
        Ok(library)
    }
}

