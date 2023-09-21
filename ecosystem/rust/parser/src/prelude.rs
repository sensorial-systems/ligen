pub use ligen_common::*;
pub use ligen_utils::prelude::*;
pub use proc_macro2::TokenStream;
pub use quote::{quote, TokenStreamExt};
pub use syn::parse_quote::parse;
pub(crate) use ligen_parsing::Parser;

// TODO: Move these to new_types.rs
macro_rules! new_type {
    ($old:ty, $i:ident) => {
        pub struct $i(pub $old);
        impl From<$i> for $old {
            fn from(value: $i) -> Self {
                value.0
            }
        }
        impl From<$old> for $i {
            fn from(value: $old) -> Self {
                Self(value)
            }
        }
    };
}

pub(crate) use new_type;
new_type!(syn::Field, SynField);
new_type!(syn::ItemStruct, SynItemStruct);

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
