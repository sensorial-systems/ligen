use std::path::PathBuf;

// TODO: Base this on Config (which will be also available in ParserConfig)
#[derive(Debug, Clone, Default)]
pub struct GeneratorConfig {
    pub path: PathBuf
}
