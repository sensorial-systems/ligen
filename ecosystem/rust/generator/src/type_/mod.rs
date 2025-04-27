use anyhow::Context;
pub use ligen_generator::prelude::*;
use ligen_ir::Type;

use crate::RustPathGenerator;



#[derive(Default)]
pub struct RustTypeGenerator {
    path_generator: RustPathGenerator,
}

impl Generator<&Type, syn::Type> for RustTypeGenerator {
    fn generate(&self, type_: &Type, _config: &Config) -> Result<syn::Type> {
        if type_.is_mutable_reference() || type_.is_constant_reference() {
            let and_token = syn::token::And::default();
            let lifetime = Default::default();
            let mutability = type_.is_mutable_reference().then_some(Default::default());
            let type_ = type_
                .path
                .segments
                .last()
                .context("Type path is empty")?
                .generics
                .types
                .last()
                .context("Type generic arguments are empty")?;
            let type_ = self.generate(type_, _config)?;
            let elem = Box::new(type_);
            Ok(syn::Type::Reference(syn::TypeReference { and_token, lifetime, mutability, elem }))    
        } else {
            let qself = None;
            let path = self.path_generator.generate(&type_.path, _config)?;
            let type_path = syn::TypePath { qself, path };
            Ok(syn::Type::Path(type_path))
        }
    }
}
