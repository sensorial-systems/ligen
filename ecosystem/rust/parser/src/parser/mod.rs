use ligen::prelude::*;
use ligen::idl::Registry;

#[derive(Default)]
pub struct RustParser;

impl RustParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<&std::path::Path, Registry> for RustParser {
    fn transform(&self, _input: &std::path::Path, _config: &Config) -> Result<Registry> {
        let registry = Registry::new();

        Ok(registry)
    }

    fn name(&self) -> &str {
        "Rust"
    }

    fn config(&self) -> Config {
        Default::default()
    }
}