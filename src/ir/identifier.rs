use quote::{quote, ToTokens, TokenStreamExt};

use proc_macro2::TokenStream;

/// Identifier structure
#[derive(Clone, Debug)]
pub struct Identifier {
    /// Name field of Identifier
    pub name: String,
}

impl Identifier {
    /// Create a new Identifier
    pub fn new(name: &str) -> Self {
        let name = String::from(name);
        Self { name }
    }
}

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
        let identifier: Identifier = identifier.into();
        assert_eq!(identifier.name, "id");
    }
}
