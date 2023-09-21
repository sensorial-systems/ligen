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
        let token_stream = proc_macro2::TokenStream::from(input);
        self.parse(token_stream)
    }
}

impl Parser<proc_macro2::TokenStream> for PathParser {
    type Output = Path;
    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        self.parse(syn::parse2::<syn::Path>(input).expect("Failed to parse Path."))
    }
}

impl ToTokens for Path {
    fn to_tokens(&self, tokens: &mut TokenStream) {
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
    use ligen_ir::{Path, Identifier};
    use ligen_parsing::Parser;
    use crate::path::PathParser;
    use crate::prelude::*;

    #[test]
    fn identifier_as_path() -> Result<()> {
        let path: Path = PathParser.parse(quote! { u8 })?;
        assert_eq!(path.segments.first(), Some(&Identifier::new("u8")));
        Ok(())
    }

    #[test]
    fn path() -> Result<()> {
        let path: Path = PathParser.parse(quote! { std::convert::TryFrom })?;
        let segments: Vec<_> = vec!["std", "convert", "TryFrom"].into_iter().map(Identifier::from).collect();
        assert_eq!(path.segments, segments);
        Ok(())
    }

    #[test]
    fn path_from_string() {
        let path: Path = "std::convert::TryFrom".into();
        let segments: Vec<_> = vec!["std", "convert", "TryFrom"].into_iter().map(Identifier::from).collect();
        assert_eq!(path.segments, segments);
    }

}