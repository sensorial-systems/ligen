use ligen_generator::prelude::*;
use ligen_generator::{Generator, GeneratorConfig};
use anyhow::Context;

#[derive(Debug, Default)]
pub struct AnchorGenerator;

impl AnchorGenerator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Generator<ligen_ir::Library> for AnchorGenerator {
    type Output = anchor_lang_idl_spec::Idl;
    fn generate(&self, input: &ligen_ir::Library, _config: &GeneratorConfig) -> Result<Self::Output> {
        let address = input.metadata.table.get("address").context("Address not found in metadata.")?.to_string();
        let contact = if input.metadata.authors.is_empty() {
            None
        } else {
            Some(input.metadata.authors.iter().map(|author| author.to_string()).collect::<Vec<_>>().join(", "))
        };
        let name = input.identifier.to_string();
        let description = input.metadata.description.clone();
        let repository = input.metadata.homepage.clone();
        let version = input.metadata.version.to_string();
        let dependencies = Default::default();
        let anchor_idl = anchor_lang_idl_spec::Idl {
            address,
            metadata: anchor_lang_idl_spec::IdlMetadata {
                contact,
                description,
                name,
                repository,
                spec: anchor_lang_idl_spec::IDL_SPEC.to_string(),
                version,
                dependencies,
                deployments: None,
            },
            docs: vec![],
            instructions: vec![],
            accounts: vec![],
            events: vec![],
            errors: vec![],
            types: vec![],
            constants: vec![],
        };
        Ok(anchor_idl)
    }
}