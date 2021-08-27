use quote::{quote, ToTokens, TokenStreamExt};
use crate::prelude::*;

use proc_macro2::TokenStream;
use crate::conventions::naming::SnakeCase;

/// Identifier structure
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display)]
#[display(fmt = "{}", name)]
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

impl From<SnakeCase> for Identifier {
    fn from(snake_case: SnakeCase) -> Self {
        snake_case.to_string().into()
    }
}

impl From<&str> for Identifier {
    fn from(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl From<String> for Identifier {
    fn from(name: String) -> Self {
        name.as_str().into()
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
