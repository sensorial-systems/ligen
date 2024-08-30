pub(crate) enum IntermediaryAttribute {
    Meta(syn::Meta),
    Lit(syn::Lit),
    Expr(syn::Expr),
    Unknown(String)
}

impl syn::parse::Parse for IntermediaryAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Lit) {
            input.parse().map(IntermediaryAttribute::Lit)
        } else {
            if let Ok(attribute) = input.parse().map(IntermediaryAttribute::Expr) {
                Ok(attribute)
            } else {
                Ok(input.parse().map(IntermediaryAttribute::Meta).unwrap_or(IntermediaryAttribute::Unknown(input.to_string())))
            }
        }
    }
}
