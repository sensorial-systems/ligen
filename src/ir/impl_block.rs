use crate::ir::{Attribute, Attributes, Identifier};
use syn::ItemImpl;

#[derive(Debug, PartialEq)]
/// Function Struct
pub struct Impl {
    /// attributes field
    pub attributes: Attributes,
    /// self_ field
    pub self_: Identifier,
    /// items field
    pub items: Option<()>,
}

impl From<ItemImpl> for Impl {
    fn from(item_impl: ItemImpl) -> Self {
        println!("{:#?}", item_impl);

        if let syn::Type::Path(syn::TypePath { path, .. }) = *item_impl.self_ty {
            Self {
                attributes: Attributes {
                    attributes: item_impl
                        .attrs
                        .into_iter()
                        .map(|x| Attribute::from(x.parse_meta().expect("Failed to parse Meta")))
                        .collect(),
                },
                self_: Identifier::from(path.segments[0].ident.clone()),
                items: None,
            }
        } else {
            panic!("Impl Block Identifier not found")
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Attribute, Attributes, Identifier, Impl, ItemImpl};
    use crate::ir::Literal;
    use quote::quote;
    use syn::parse_quote::parse;

    #[test]
    fn impl_block() {
        assert_eq!(
            Impl::from(parse::<ItemImpl>(quote! {impl Test {}})),
            Impl {
                attributes: Attributes { attributes: vec![] },
                self_: Identifier {
                    name: String::from("Test")
                },
                items: None
            }
        );
    }

    #[test]
    fn impl_block_attributes() {
        assert_eq!(
            Impl::from(parse::<ItemImpl>(quote! {
                #[test(a = "b")]
                impl Test {}
            })),
            Impl {
                attributes: Attributes {
                    attributes: vec![Attribute::Group(
                        Identifier::new("test"),
                        Attributes {
                            attributes: vec![Attribute::Named(
                                Identifier::new("a"),
                                Literal::String(String::from("b"))
                            )]
                        }
                    )]
                },
                self_: Identifier {
                    name: String::from("Test")
                },
                items: None
            }
        );
    }
}
