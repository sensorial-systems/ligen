pub use ligen_ir::prelude::*;
pub(crate) use shrinkwraprs::Shrinkwrap;
pub(crate) use proc_macro2::TokenStream;
pub(crate) use quote::{quote, ToTokens, TokenStreamExt};

macro_rules! new_type {
    ($name:ident) => {
        #[derive(Shrinkwrap)]
        pub struct name(pub syn::$name);
    }
}

pub mod syn {
    new_type!(Path);
    new_type!(Item);
    new_type!(Attribute);
    new_type!(ImplItemConst);
    new_type!(ItemConst);
    new_type!(FnArg);
    new_type!(Visibility);
    new_type!(Ident);
    new_type!(PathArguments);
}