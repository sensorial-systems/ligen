use ligen_ir::Path;
use ligen_parsing::Parser;
use crate::identifier::IdentifierParser;
use crate::prelude::*;

pub struct PathParser;

impl Parser<syn::Path> for PathParser {
    type Output = Path;
    fn parse(&self, path: syn::Path) -> Result<Self::Output> {
        let segments = path
            .segments
            .iter()
            .map(|segment| IdentifierParser.parse(segment.ident.clone()).expect("Failed to parse segment."))
            .collect();
        Ok(Self::Output { segments })
    }
}

impl Parser<syn::Ident> for PathParser {
    type Output = Path;
    fn parse(&self, identifier: syn::Ident) -> Result<Self::Output> {
        let segments = vec![IdentifierParser.parse(identifier)?];
        Ok(Self::Output { segments })
    }
}

impl Parser<proc_macro::TokenStream> for PathParser {
    type Output = Path;
    fn parse(&self, input: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(input))
    }
}

impl Parser<proc_macro2::TokenStream> for PathParser {
    type Output = Path;
    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::Path>(input)
            .map_err(|e| Error::Message(format!("Failed to parse path: {:?}", e)))
            .and_then(|path| self.parse(path))
    }
}

impl ToTokens for Path {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut segments = self.segments.iter();
        tokens.append_all(segments.next().unwrap().to_token_stream());
        for segment in segments {
            let segment = segment.to_token_stream();
            tokens.append_all(quote! { ::#segment })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::path::PathParser;
    use crate::prelude::*;

    use ligen_parsing::assert::*;
    use ligen_ir::path::mock;

    #[test]
    fn identifier_as_path() -> Result<()> {
        assert_eq(PathParser, mock::identifier_as_path(), quote! {
            u8
        })
    }

    #[test]
    fn path() -> Result<()> {
        assert_eq(PathParser, mock::path(), quote! {
            std::convert::TryFrom
        })
    }
}