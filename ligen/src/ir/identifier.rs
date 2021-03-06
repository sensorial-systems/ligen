use quote::{quote, ToTokens, TokenStreamExt};

use proc_macro2::TokenStream;

/// Identifier structure
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Identifier {
    /// Name field of Identifier
    pub name: String,
}

impl Identifier {
    /// Create a new Identifier
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        let name = String::from(name.as_ref());
        Self { name }
    }
}

impl From<&str> for Identifier {
    fn from(name: &str) -> Self {
        Self { name: name.to_string() }
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

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.name))
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
