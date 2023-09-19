use ligen_ir::{Mutability, Reference};
use crate::prelude::*;

impl ToTokens for Reference {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.mutability {
            Mutability::Constant => tokens.append_all(quote! {*const }),
            Mutability::Mutable => tokens.append_all(quote! {*mut })
        }
        let type_ = &self.type_.to_token_stream();
        tokens.append_all(quote! {#type_});
    }
}
