use ligen::prelude::*;
use ligen::parser::{Parser, ParserConfig};
use ligen::ir::Registry;

#[derive(Default)]
pub struct RustParser;

impl RustParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<&std::path::Path> for RustParser {
    type Output = Registry;
    fn parse(&self, _input: &std::path::Path, _config: &ParserConfig) -> Result<Self::Output> {
        let registry = Registry::new();

        Ok(registry)
    }

    fn name(&self) -> &str {
        "Rust"
    }

    fn config(&self) -> ParserConfig {
        Default::default()
    }
}