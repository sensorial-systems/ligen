pub use ligen_ir::prelude::*;
pub(crate) use shrinkwraprs::Shrinkwrap;
pub(crate) use proc_macro2::TokenStream;
pub(crate) use quote::{quote, ToTokens, TokenStreamExt};

pub mod proc_macro {
    use super::*;

    macro_rules! new_type {
        ($name:ident) => {
            #[derive(Shrinkwrap)]
            pub struct $name(pub original_proc_macro::$name);
        }
    }
    new_type!(TokenStream);
}

pub mod syn {
    use super::*;

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
    new_type!(ImplItemMethod);
    new_type!(ItemFn);
    new_type!(Lit);
    new_type!(TypePtr);
    new_type!(TypeReference);
    new_type!(UseTree);
    new_type!(Meta);
    new_type!(Expr);
    new_type!(Pat);
    new_type!(ReturnType);
    new_type!(ImplItem);
    new_type!(GenericArgument);
    new_type!(ItemUse);
    new_type!(ItemMod);
    new_type!(ItemMacro);
    new_type!(MetaList);
    new_type!(MetaNameValue);
    pub use original_syn::{parse_quote, parse, parse2, Token, parse_file};
}