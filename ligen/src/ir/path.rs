use crate::ir::Identifier;
use quote::ToTokens;
use quote::TokenStreamExt;
use quote::quote;
use proc_macro2::TokenStream;

/// A fully qualified path.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Path {
    /// The path segments.
    pub segments: Vec<Identifier>
}

impl Path {
    /// Get the last segment of the path.
    pub fn last(&self) -> Identifier {
        self.segments.last().unwrap().clone()
    }

    /// Join the current path with another path.
    pub fn join<T: Into<Path>>(&mut self, another: T) {
        self.segments.append(&mut another.into().segments);
    }
}

impl<I: Into<Identifier>> From<Vec<I>> for Path {
    fn from(from: Vec<I>) -> Self {
        let segments = from.into_iter().map(|x| x.into()).collect();
        Self { segments }
    }
}

impl From<&str> for Path {
    fn from(string: &str) -> Path {
        let segments = string
            .split("::")
            .into_iter()
            .map(|segment| Identifier::new(segment))
            .collect();
        Self { segments }
    }
}

impl From<String> for Path {
    fn from(string: String) -> Path {
        string.as_str().into()
    }
}

impl From<syn::Path> for Path {
    fn from(path: syn::Path) -> Self {
        let segments = path
            .segments
            .iter()
            .map(|segment| segment.ident.clone().into())
            .collect();
        Self { segments }
    }
}

impl From<Identifier> for Path {
    fn from(identifier: Identifier) -> Self {
        let segments = vec![identifier];
        Self { segments }
    }
}

impl From<syn::Ident> for Path {
    fn from(identifier: syn::Ident) -> Self {
        let segments = vec![identifier.into()];
        Self { segments }
    }
}

impl ToTokens for Path {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut segments = self.segments.iter();
        tokens.append_all(segments.next().unwrap().to_token_stream());
        for segment in segments {
            tokens.append_all(quote! { ::#segment })
        }
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let segments: Vec<_> = self.segments.iter().map(|identifier| identifier.to_string()).collect();
        f.write_str(&segments.join(", "))
    }
}

#[cfg(test)]
mod test {
    use quote::quote;
    use syn::parse_quote::parse;
    use crate::ir::Path;
    use crate::ir::Identifier;

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