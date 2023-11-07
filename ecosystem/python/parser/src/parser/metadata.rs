use ligen::parser::ParserConfig;
use ligen::ir::{Metadata, Version, VersionRequirement, Author, Dependency, Language};

use crate::prelude::*;

#[derive(Default)]
pub struct MetadataParser {}

impl MetadataParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<python_pkginfo::Metadata> for MetadataParser {
    type Output = Metadata;
    fn parse(&self, input: python_pkginfo::Metadata, _config: &ParserConfig) -> Result<Self::Output> {
        let version = Version::try_from(input.version.as_str())?;
        let requirement = VersionRequirement::try_from(input.requires_python.unwrap_or_default().as_str())?;
        let language = Language { name: "Python".into(), requirement };
        let homepage = input.home_page.unwrap_or_default();
        let summary = input.summary.unwrap_or_default();
        let description = input.description.unwrap_or_default();
        let keywords = input.keywords.unwrap_or_default().split(',').map(String::from).collect();
        let authors = vec![Author::new(input.author.unwrap_or_default(), input.author_email.unwrap_or_default())];
        let license = input.license.unwrap_or_default();
        let mut dependencies = Vec::new();
        for requirement in input.requires_dist {
            let requirement = Dependency::try_from(requirement.as_str())?;
            dependencies.push(requirement);
        }
        Ok(Self::Output { version, authors, dependencies, keywords, description, language, homepage, summary, license })
    }
}

impl Parser<&std::path::Path> for MetadataParser {
    type Output = Metadata;
    fn parse(&self, input: &std::path::Path, config: &ParserConfig) -> Result<Self::Output> {
        let name = input.file_name().ok_or("Failed to get file name.")?;
        let name = name.to_string_lossy().to_string();
        let input = input.parent().ok_or("Failed to get parent.")?;
        let dir = input.read_dir()?;
        let dist_info_dir = dir
            .filter_map(|entry| entry.ok())
            .find(|entry| {
                let file_name = entry
                    .file_name()
                    .to_string_lossy()
                    .to_string();
                file_name.starts_with(&name) && file_name.ends_with(".dist-info")
            });
        let metadata_file = dist_info_dir
            .ok_or("Failed to find metadata file.")?
            .path()
            .join("METADATA");
        let content = std::fs::read_to_string(metadata_file)?;
        let metadata = python_pkginfo::Metadata::parse(content.as_bytes())
            .map_err(|e| Error::Message(format!("Failed to parse metadata: {}", e)))?;
        self.parse(metadata, config)
    }
}