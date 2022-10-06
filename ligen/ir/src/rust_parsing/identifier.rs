use crate::prelude::*;

use proc_macro2::TokenStream;
use crate::Identifier;

impl From<syn::Ident> for Identifier {
    fn from(ident: syn::Ident) -> Self {
        let name = ident.to_string();
        Self { name }
    }
}

impl ToTokens for Identifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let identifier = proc_macro2::Ident::new(&self.name, proc_macro2::Span::call_site());
        tokens.append_all(quote! {
            #identifier
        });
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
        let identifier = Identifier::from(identifier);
        assert_eq!(identifier.name, "id");
    }
}
