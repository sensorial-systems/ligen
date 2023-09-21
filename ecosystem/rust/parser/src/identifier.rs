use crate::prelude::*;

use ligen_ir::Identifier;

impl From<SynIdent> for Identifier {
    fn from(SynIdent(ident): SynIdent) -> Self {
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
    use crate::prelude::SynIdent;

    #[test]
    fn identifier() {
        let tokenstream = quote! { id };
        let identifier: syn::Ident = parse(tokenstream);
        let identifier = Identifier::from(SynIdent(identifier));
        assert_eq!(identifier.name, "id");
    }
}
