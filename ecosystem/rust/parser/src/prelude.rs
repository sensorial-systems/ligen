pub use ligen_common::*;
pub use ligen_utils::prelude::*;
pub use proc_macro2::TokenStream;
pub use quote::{quote, TokenStreamExt};
pub(crate) use ligen_parsing::Parser;

pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream);
    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }
}

impl quote::ToTokens for dyn ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (self as &dyn ToTokens).to_tokens(tokens);
    }
}
