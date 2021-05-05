use crate::ir::Attributes;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::parse2;
use crate::ir::{Attribute, Identifier, Literal};

const PREFIX: &'static str = "ligen_";

/// `ligen` entry-point called by `#[ligen]`.
pub fn ligen(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse2::<Attributes>(args).expect("Failed to parse Attributes");

    let mut attributes = TokenStream::new();

    let macro_attributes = args
        .attributes
        .into_iter()
        .map(to_ligen_macro);

    macro_attributes.for_each(|macro_attribute|
        attributes.append_all(quote! { #macro_attribute })
    );

    let tokenstream = quote! {
        #attributes
        #item
    };

    tokenstream
}

/// Convert Attribute to a Ligen Macro attribute
pub fn to_ligen_macro(attribute: Attribute) -> Attribute {
    match attribute {
        Attribute::Literal(literal) => {
            Attribute::Literal(Literal::String(format!("{}{}", PREFIX, literal)))
        }
        Attribute::Named(ident, lit) => Attribute::Named(ident, lit),
        Attribute::Group(ident, group) => Attribute::Group(
            Identifier::new(format!("{}{}", PREFIX, ident.name).as_str()),
            Attributes {
                attributes: group
                    .attributes
                    .into_iter()
                    .filter_map(|x| {
                        if let Attribute::Named(ident, lit) = x {
                            Some(Attribute::Named(ident, lit))
                        } else {
                            None
                        }
                    })
                    .collect(),
            },
        ),
    }
}


#[cfg(test)]
mod test {
    use super::ligen;
    use quote::quote;

    #[test]
    fn ligen_main() {
        assert_eq!(
            quote! {
                #[ligen_c(int = "sized")]
                #[ligen_python]
                struct Test;
            }
                .to_string(),
            ligen(quote! {c(int = "sized"), python}, quote! {struct Test;}).to_string()
        );
    }
}
