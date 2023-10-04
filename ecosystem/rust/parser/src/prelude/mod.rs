pub use ligen::common::*;
pub use ligen::utils::prelude::*;
pub use quote::{quote, TokenStreamExt};
pub(crate) use ligen::parsing::Parser;

pub mod syn2;

pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream);
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }
}

impl quote::ToTokens for dyn ToTokens {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        (self as &dyn ToTokens).to_tokens(tokens);
    }
}
