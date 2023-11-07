pub struct Punctuated<T, P>(pub syn::punctuated::Punctuated<T, P>);

impl<T: syn::parse::Parse, P: syn::parse::Parse> syn::parse::Parse for Punctuated<T, P> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        syn::punctuated::Punctuated::<T, P>::parse_terminated(input)
            .map(|punctuated| Self(punctuated))
    }
}
