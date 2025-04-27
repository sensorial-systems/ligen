use ligen_transformer::prelude::*;
use ligen_ir::Library;

#[derive(Default)]
pub struct LlmLibraryParser {
    
}

impl Transformer<&std::path::Path, Library> for LlmLibraryParser {
    fn transform(&self, _input: &std::path::Path, _config: &Config) -> Result<Library> {
        Ok(Library::default())
    }
}
