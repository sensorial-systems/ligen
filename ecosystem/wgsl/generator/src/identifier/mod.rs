use ligen_transformer::prelude::*;
use ligen_ir::Identifier;

#[derive(Default)]
pub struct WgslIdentifierGenerator;

impl Generator<&Identifier, String> for WgslIdentifierGenerator {
    fn generate(&self, identifier: &Identifier, _config: &Config) -> Result<String> {
        Ok(identifier.to_string())
    }
}
