use ligen::parser::ParserConfig;
use ligen::ir::{Metadata, Version};

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
        Ok(Self::Output { version })
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
        let content = std::fs::read_to_string(&metadata_file)?;
        let metadata = python_pkginfo::Metadata::parse(content.as_bytes())
            .map_err(|e| Error::Message(format!("Failed to parse metadata: {}", e)))?;
        self.parse(metadata, config)
    }
}