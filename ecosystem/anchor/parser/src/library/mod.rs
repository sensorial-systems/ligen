use anchor_lang_idl_spec::Idl;
use ligen_ir::{prelude::Result, Author, Dependency, Identifier, Language, Library, Metadata, Version, VersionRequirement};
use ligen_transformer::prelude::*;

use crate::module::ModuleParser;


#[derive(Default)]
pub struct LibraryParser {
    module_parser: ModuleParser,
}

impl Transformer<&std::path::Path, Library> for LibraryParser {
    fn transform(&self, input: &std::path::Path, config: &Config) -> Result<Library> {
        let input = std::fs::read_to_string(input)?;
        let input = serde_json::from_str::<Idl>(&input)?;
        self.transform(input, config)
    }

    fn name(&self) -> &str {
        "Anchor IDL Parser"
    }
}

impl Transformer<anchor_lang_idl_spec::Idl, Library> for LibraryParser {
    fn transform(&self, input: Idl, config: &Config) -> Result<Library> {
        let identifier = Identifier::new(input.metadata.name.clone());
        let authors = input
            .metadata
            .contact
            .as_ref()
            .map(|contact| Author {
                name: Default::default(),
                email: contact.into(),
            }).iter()
            .cloned()
            .collect();
        let language = Language {
            name: "Anchor IDL".to_string(),
            requirement: Some(VersionRequirement::from(input.metadata.spec.clone())),
        };
        let version = Version::try_from(input.metadata.version.clone())?;
        let summary = Default::default();
        let description = input.metadata.description.clone();
        let homepage = input.metadata.repository.clone();
        let dependencies = input
            .metadata
            .dependencies
            .iter()
            .map(|dependency| Dependency {
                identifier: Identifier::new(dependency.name.clone()),
                requirement: VersionRequirement::from(dependency.version.clone()),
                features: Default::default(),
            })
            .collect();
        let keywords = Default::default();
        let license = Default::default();
        let table = [("address".to_string(), input.address.clone())].into_iter().collect();
        let metadata = Metadata { authors, version, language, summary, description, homepage, dependencies, keywords, license, table };
        let root_module = self.module_parser.transform(input, config)?;
        let library = Library { identifier, metadata, root_module };
        Ok(library)
    }

    fn name(&self) -> &str {
        "Anchor IDL Parser"
    }
}
