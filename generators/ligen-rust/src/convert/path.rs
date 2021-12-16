use crate::Identifier;
use crate::prelude::*;
use ligen_ir::Path;
use crate::traits::AsRust;

impl From<syn::Path> for Path {
    fn from(path: syn::Path) -> Self {
        let segments = path.0
            .segments
            .iter()
            .map(|segment| syn::Ident(segment.ident.clone()).into())
            .collect();
        Self { segments }
    }
}

impl From<syn::Ident> for Path {
    fn from(identifier: syn::Ident) -> Self {
        let segments = vec![identifier.into()];
        Self { segments }
    }
}

impl AsRust for Path {
    fn as_rust(&self) -> String {
        let segments: Vec<_> = self.segments.iter().map(|identifier| identifier.to_string()).collect();
        segments.join("::")
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