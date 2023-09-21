use ligen_ir::Synchrony;
use ligen_parsing::Parser;
use crate::prelude::*;

pub struct SynchronyParser;

impl Parser<Option<syn::token::Async>> for SynchronyParser {
    type Output = Synchrony;
    fn parse(&self, input: Option<syn::token::Async>) -> Result<Self::Output> {
        Ok(match input {
            Some(_) => Synchrony::Asynchronous,
            None => Synchrony::Synchronous,
        })
    }
}
