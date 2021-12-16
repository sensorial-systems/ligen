pub use ligen_ir::prelude::*;
pub(crate) use shrinkwraprs::Shrinkwrap;
pub(crate) use proc_macro2::TokenStream;
pub(crate) use quote::{quote, ToTokens, TokenStreamExt};

pub mod proc_macro {
    macro_rules! new_type {
        ($name:ident) => {
            #[derive(Shrinkwrap)]
            pub struct $name(pub original_proc_macro::$name);
        }
    }
    new_type!(TokenStream);
}

pub mod syn {
    macro_rules! new_type {
        ($name:ident) => {
            #[derive(Shrinkwrap)]
            pub struct $name(pub original_syn::$name);
        }
    }

    new_type!(Path);
    new_type!(Item);
    new_type!(Attribute);
    new_type!(ImplItemConst);
    new_type!(ItemConst);
    new_type!(FnArg);
    new_type!(Visibility);
    new_type!(Ident);
    new_type!(PathArguments);
    new_type!(Type);
    new_type!(ItemEnum);
    new_type!(Variant);
    new_type!(ItemStruct);
    new_type!(Field);
    new_type!(TypePath);
    new_type!(ItemImpl);
    new_type!(NestedMeta);
    new_type!(AttributeArgs);
    new_type!(Attribute);
    pub use original_syn::{parse, parse2};
}