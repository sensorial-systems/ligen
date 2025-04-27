use ligen_generator::prelude::*;
use ligen_ir::prelude::Result;

#[derive(Debug, Default)]
pub struct AnchorMetadataGenerator;

impl AnchorMetadataGenerator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Generator<&ligen_ir::Metadata, anchor_lang_idl_spec::IdlMetadata> for AnchorMetadataGenerator {
    fn generate(&self, input: &ligen_ir::Metadata, _config: &Config) -> Result<anchor_lang_idl_spec::IdlMetadata> {
        let name = Default::default();
        let contact = if input.authors.is_empty() {
            None
        } else {
            Some(input.authors.iter().map(|author| author.to_string()).collect::<Vec<_>>().join(", "))
        };
        let description = input.description.clone();
        let repository = input.homepage.clone();
        let version = input.version.to_string();
        let dependencies = Default::default();
        let metadata = anchor_lang_idl_spec::IdlMetadata {
            contact,
            description,
            name,
            repository,
            spec: anchor_lang_idl_spec::IDL_SPEC.to_string(),
            version,
            dependencies,
            deployments: None,
        };
        Ok(metadata)
    }
}
