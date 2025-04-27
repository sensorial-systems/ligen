use ligen_ir::{attribute::Named, prelude::Result, Attribute, Attributes};
use ligen_parser::prelude::*;

#[derive(Default)]
pub struct DocParser;

impl Parser<Vec<String>> for DocParser {
    type Output = Attributes;

    fn parse(&self, input: Vec<String>, _config: &Config) -> Result<Self::Output> {
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
