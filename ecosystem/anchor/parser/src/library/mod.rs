use anchor_lang_idl_spec::Idl;
use ligen_ir::{prelude::Result, Author, Dependency, Identifier, Language, Library, Metadata, Version, VersionRequirement};
use ligen_parser::{Parser, ParserConfig};

use crate::module::ModuleParser;


#[derive(Default)]
pub struct LibraryParser {
    module_parser: ModuleParser,
}

impl Parser<&std::path::Path> for LibraryParser {
    type Output = Library;

    fn parse(&self, input: &std::path::Path, config: &ParserConfig) -> Result<Self::Output> {
        let input = std::fs::read_to_string(input)?;
        let input = serde_json::from_str::<Idl>(&input)?;
        self.parse(input, config)
    }

    fn name(&self) -> &str {
        "Anchor IDL Parser"
    }
}

impl Parser<anchor_lang_idl_spec::Idl> for LibraryParser {
    type Output = Library;
    
    fn parse(&self, input: Idl, config: &ParserConfig) -> Result<Self::Output> {
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
            requirement: VersionRequirement::from(input.metadata.spec.clone()),
        };
        let version = Version::try_from(input.metadata.version.clone())?;
        let summary = Default::default();
        let description = input.metadata.description.clone().unwrap_or_default();
        let homepage = input.metadata.repository.clone().unwrap_or_default();
        let dependencies = input
            .metadata
            .dependencies
            .iter()
            .map(|dependency| Dependency {
                identifier: Identifier::new(dependency.name.clone()),
                requirement: VersionRequirement::from(dependency.version.clone()),
                feature: Default::default(),
            })
            .collect();
        let keywords = Default::default();
        let license = Default::default();
        let metadata = Metadata { authors, version, language, summary, description, homepage, dependencies, keywords, license };
        let root_module = self.module_parser.parse(input, config)?;
        let library = Library { identifier, metadata, root_module };
        Ok(library)
    }

    fn name(&self) -> &str {
        "Anchor IDL Parser"
    }
}
