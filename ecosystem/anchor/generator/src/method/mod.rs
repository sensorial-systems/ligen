use anyhow::Context;
use ligen_transformer::prelude::*;
use sha2::{Sha256, Digest};

use crate::AnchorTypeGenerator;

#[derive(Debug, Default)]
pub struct AnchorMethodGenerator {
    pub type_generator: AnchorTypeGenerator,
}

impl AnchorMethodGenerator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Generator<&ligen_ir::Method, anchor_lang_idl_spec::IdlInstruction> for AnchorMethodGenerator {
    fn generate(&self, method: &ligen_ir::Method, _config: &Config) -> Result<anchor_lang_idl_spec::IdlInstruction> {
        let name = method.identifier.to_string();
        let docs = method.attributes.get_documentation();
        let discriminator = Sha256::digest(format!("global:{name}").as_bytes())[..8].to_vec();
        let returns = method.output.as_ref().and_then(|output| self.type_generator.generate(output, &Config::default()).ok());
        let mut accounts = Vec::new();
        let mut args = Vec::new();

        for input in method.inputs.iter() {
            let (type_, optional) = if input.type_.is_option() {
                let ty = input.type_.path.last().generics.types.first().context("Expected a type in the option")?;
                (ty, true)
            } else {
                (&input.type_, false)
            };
            let account = if type_.is_mutable_reference() {
                Some(true)
            } else if input.type_.is_constant_reference() {
                Some(false)
            } else {
                None
            };
            if let Some(writable) = account {
                let name = input.identifier.to_string();
                let type_ = type_.path.last().generics.types.first().context("Expected a type in the reference")?;
                let signer = type_.is("Signer");
                let docs = input.attributes.get_documentation();
                let address = Default::default();
                let pda = Default::default();
                let relations = Default::default();
                accounts.push(anchor_lang_idl_spec::IdlInstructionAccountItem::Single(anchor_lang_idl_spec::IdlInstructionAccount { name, docs, writable, signer, optional, address, pda, relations }));
            } else {
                let ty = self.type_generator.generate(&input.type_, &Config::default()).unwrap();
                let name = input.identifier.to_string();
                let docs = input.attributes.get_documentation();
                args.push(anchor_lang_idl_spec::IdlField { name, ty, docs  });
            }
        }
           

        let instruction = anchor_lang_idl_spec::IdlInstruction { name, docs, discriminator, accounts, args, returns };
        Ok(instruction)
    }
}