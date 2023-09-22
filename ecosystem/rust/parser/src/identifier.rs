use crate::prelude::*;

use ligen_ir::Identifier;
use ligen_parsing::Parser;

pub struct IdentifierParser;

impl Parser<proc_macro::TokenStream> for IdentifierParser {
    type Output = Identifier;
    fn parse(&self, token_stream: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(token_stream))
    }
}

impl Parser<proc_macro2::TokenStream> for IdentifierParser {
    type Output = Identifier;
    fn parse(&self, token_stream: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::Ident>(token_stream)
            .map_err(|e| Error::Message(format!("Failed to parse identifier: {:?}", e)))
            .and_then(|ident| self.parse(ident))
    }
}


impl Parser<syn::Ident> for IdentifierParser {
    type Output = Identifier;
    fn parse(&self, ident: syn::Ident) -> Result<Self::Output> {
        let name = ident.to_string();
        Ok(Self::Output { name })
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
    use ligen_parsing::Parser;
    use crate::identifier::IdentifierParser;
    use crate::prelude::*;

    #[test]
    fn identifier() -> Result<()> {
        let identifier = IdentifierParser.parse(quote! { id })?;
        assert_eq!(identifier.name, "id");
        Ok(())
    }
}
