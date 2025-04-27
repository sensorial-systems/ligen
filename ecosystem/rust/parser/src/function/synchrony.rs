use ligen::ir::Synchrony;
use ligen::parser::prelude::*;

#[derive(Default)]
pub struct SynchronyParser;

impl Transformer<Option<syn::token::Async>, Synchrony> for SynchronyParser {
    fn transform(&self, input: Option<syn::token::Async>, _config: &Config) -> Result<Synchrony> {
        Ok(match input {
            Some(_) => Synchrony::Asynchronous,
            None => Synchrony::Synchronous,
        })
    }
}
