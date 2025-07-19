use ligen::idl::Synchrony;
use ligen::transformer::prelude::*;

#[derive(Default)]
pub struct RustSynchronyParser;

impl Transformer<Option<syn::token::Async>, Synchrony> for RustSynchronyParser {
    fn transform(&self, input: Option<syn::token::Async>, _config: &Config) -> Result<Synchrony> {
        Ok(match input {
            Some(_) => Synchrony::Asynchronous,
            None => Synchrony::Synchronous,
        })
    }
}
