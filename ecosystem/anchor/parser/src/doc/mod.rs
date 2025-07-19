use ligen_idl::prelude::*;
use ligen_idl::{Named, Attribute, Attributes};
use ligen_transformer::prelude::*;

#[derive(Default)]
pub struct DocParser;

impl Transformer<Vec<String>, Attributes> for DocParser {
    fn transform(&self, input: Vec<String>, _config: &Config) -> Result<Attributes> {
        let attributes = input
            .iter()
            .map(|doc| Attribute::from(Named::new("doc", doc.clone())))
            .collect::<Vec<_>>()
            .into();
        Ok(attributes)
    }

    fn name(&self) -> &str {
        "Anchor IDL Doc Parser"
    }
}
