use is_tree::HasBranchesAPIV2;
use ligen_generator::prelude::*;
use anyhow::Context;

use crate::{AnchorMetadataGenerator, AnchorMethodGenerator, AnchorTypeDefinitionGenerator};

#[derive(Debug, Default)]
pub struct AnchorGenerator {
    metadata_generator: AnchorMetadataGenerator,
    type_definition_generator: AnchorTypeDefinitionGenerator,
    method_generator: AnchorMethodGenerator,
}

impl AnchorGenerator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Generator<&ligen_ir::Library, anchor_lang_idl_spec::Idl> for AnchorGenerator {
    fn generate(&self, input: &ligen_ir::Library, config: &Config) -> Result<anchor_lang_idl_spec::Idl> {
        let mut metadata = self.metadata_generator.generate(&input.metadata, config)?;
        metadata.name = input.identifier.to_string();

        let address = input.metadata.table.get("address").context("Address not found in metadata.")?.to_string();
        let docs = input.root_module.attributes.get_documentation();
        let accounts = vec![];
        let events = vec![];
        let errors = vec![];
        let constants = vec![];

        let instructions = input
            .all_branches::<&ligen_ir::Module>()
            .flat_map(|module| module.interfaces.iter())
            .filter(|interface| interface.attributes.contains("program"))
            .flat_map(|interface| &interface.methods)
            .filter_map(|method| {
                (method.identifier != "process_instruction")
                    .then(|| self.method_generator.generate(method, config).ok())
                    .flatten()
            }).collect::<Vec<_>>();

        let types = input
            .all_branches::<&ligen_ir::Module>()
            .flat_map(|module| module.types.iter())
            .filter_map(|type_def| self.type_definition_generator.generate(type_def, config).ok())
            .collect::<Vec<_>>();

        let anchor_idl = anchor_lang_idl_spec::Idl { address, metadata, docs, instructions, accounts, events, errors, types, constants };
        Ok(anchor_idl)
    }
}