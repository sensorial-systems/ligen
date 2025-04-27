use ligen::ir::Synchrony;
use ligen::parser::prelude::*;

pub struct SynchronyParser;

impl Parser<Option<syn::token::Async>> for SynchronyParser {
    type Output = Synchrony;
    fn parse(&self, input: Option<syn::token::Async>, _config: &Config) -> Result<Self::Output> {
        Ok(match input {
            Some(_) => Synchrony::Asynchronous,
            None => Synchrony::Synchronous,
        })
    }
}
