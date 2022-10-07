use crate::Path;
use crate::prelude::*;

impl From<SynPath> for Path {
    fn from(SynPath(path): SynPath) -> Self {
        let segments = path
            .segments
            .iter()
            .map(|segment| SynIdent(segment.ident.clone()).into())
            .collect();
        Self { segments }
    }
}

impl From<SynIdent> for Path {
    fn from(identifier: SynIdent) -> Self {
        let segments = vec![identifier.into()];
        Self { segments }
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

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let segments: Vec<_> = self.segments.iter().map(|identifier| identifier.to_string()).collect();
        f.write_str(&segments.join("::"))
    }
}

#[cfg(test)]
mod test {
    use quote::quote;
    use syn::parse_quote::parse;
    use crate::Path;
    use crate::Identifier;

    #[test]
    fn identifier_as_path() {
        let path: Path = parse::<syn::Path>(quote! { u8 }).into();
        assert_eq!(path.segments.first(), Some(&Identifier::new("u8")));
    }

    #[test]
    fn path() {
        let path: Path = parse::<syn::Path>(quote! { std::convert::TryFrom }).into();
        let segments: Vec<_> = vec!["std", "convert", "TryFrom"].into_iter().map(Identifier::from).collect();
        assert_eq!(path.segments, segments);
    }

    #[test]
    fn path_from_string() {
        let path: Path = "std::convert::TryFrom".into();
        let segments: Vec<_> = vec!["std", "convert", "TryFrom"].into_iter().map(Identifier::from).collect();
        assert_eq!(path.segments, segments);
    }

}