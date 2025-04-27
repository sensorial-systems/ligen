use ligen_parser::prelude::*;
use ligen_ir::Library;

#[derive(Default)]
pub struct LlmLibraryParser {
    
}

impl Parser<&std::path::Path> for LlmLibraryParser {
    type Output = Library;
    fn parse(&self, _input: &std::path::Path, _config: &Config) -> Result<Library> {
        Ok(Library::default())
    }
}
