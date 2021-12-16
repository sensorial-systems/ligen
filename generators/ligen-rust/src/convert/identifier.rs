use crate::prelude::*;

use proc_macro2::TokenStream;
use ligen_ir::Identifier;

impl From<syn::Ident> for Identifier {
    fn from(ident: syn::Ident) -> Self {
        let name = ident.to_string();
        Self { name }
    }
}

#[cfg(test)]
mod test {
    use super::quote;
    use super::Identifier;
    use syn::parse_quote::parse;

    #[test]
    fn identifier() {
        let tokenstream = quote! { id };
        let identifier: syn::Ident = parse(tokenstream);
        let identifier: Identifier = identifier.into();
        assert_eq!(identifier.name, "id");
    }
}
