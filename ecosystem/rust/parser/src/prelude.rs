pub use ligen_common::*;
pub use ligen_utils::prelude::*;
pub use proc_macro2::TokenStream;
pub use quote::{quote, TokenStreamExt};
pub use syn::parse_quote::parse;

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
new_type!(ligen_ir::Imports, LigenImports);
new_type!(syn::ItemStruct, SynItemStruct);
new_type!(syn::ItemUse, SynItemUse);
new_type!(syn::Variant, SynVariant);
new_type!(syn::ItemEnum, SynItemEnum);
new_type!(syn::Ident, SynIdent);
new_type!(syn::Type, SynType);
new_type!(syn::ItemFn, SynItemFn);
new_type!(syn::ImplItemConst, SynImplItemConst);
new_type!(syn::ImplItemMethod, SynImplItemMethod);
new_type!(syn::ItemConst, SynItemConst);
new_type!(syn::AttributeArgs, SynAttributeArgs);
new_type!(syn::PathArguments, SynPathArguments);
new_type!(syn::Visibility, SynVisibility);
new_type!(Option<syn::token::Async>, SynAsyncness);
new_type!(syn::Lit, SynLit);

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
