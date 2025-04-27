use ligen_parser::{Parser, ParserConfig};
use ligen_ir::Library;
use ligen_parser::prelude::*;

#[derive(Default)]
pub struct LlmLibraryParser {
    
}

impl Parser<&std::path::Path> for LlmLibraryParser {
    type Output = Library;
    fn parse(&self, _input: &std::path::Path, _config: &ParserConfig) -> Result<Library> {
        Ok(Library::default())
    }
}
