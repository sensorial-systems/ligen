use crate::prelude::*;
use crate::module::OpenAPIModuleParser;
use std::path::Path;

#[derive(Default)]
pub struct OpenAPILibraryParser {
    module_parser: OpenAPIModuleParser
}

impl OpenAPILibraryParser {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Transformer<&Path, Library> for OpenAPILibraryParser {
    fn transform(&self, input: &Path, config: &Config) -> Result<Library> {
        let content = std::fs::read_to_string(input)?;
        let openapi: OpenAPI = serde_json::from_str(&content)
            .map_err(|e| Error::Message(format!("Failed to parse OpenAPI JSON: {}", e)))?;
        
        let identifier = Identifier::from(openapi.info.title.replace(" ", "_").to_lowercase());
        
        let version = Version::try_from(openapi.info.version.as_str())
            .unwrap_or_else(|_| Version::default());

        let requirement = VersionRequirement::try_from(openapi.openapi.as_str()).ok();
        let language = Language::new("OpenAPI", requirement);
        let summary = openapi.info.title.clone();
        let description = openapi.info.description.clone();
        let homepage = openapi.info.contact.as_ref().and_then(|c| c.url.clone());
        let authors = openapi.info.contact.as_ref().map(|c| {
            vec![Author::new(c.name.clone().unwrap_or_default(), c.email.clone().unwrap_or_default())]
        }).unwrap_or_default();
        let license = openapi.info.license.as_ref().map(|l| l.name.clone());
        
        let metadata = Metadata {
            version,
            language,
            summary,
            description,
            homepage,
            authors,
            license,
            ..Default::default()
        };
        
        let root_module = self.module_parser.transform(&openapi, config)?;
        
        Ok(Library { identifier, metadata, root_module })
    }
}
