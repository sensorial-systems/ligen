use crate::ir::{Attribute, Attributes, Identifier};
use syn::ItemImpl;

#[derive(Debug, PartialEq)]
/// Function Struct
pub struct Impl {
    /// attributes field
    pub attributes: Attributes,
    /// trait_ field
    pub trait_: Option<Identifier>,
    /// self_ field
    pub self_: Identifier,
    /// items field
    pub items: Option<()>,
}

impl From<ItemImpl> for Impl {
    fn from(item_impl: ItemImpl) -> Self {
        println!("{:#?}", item_impl);

        let trait_ = match item_impl.trait_ {
            None => None,
            Some((Some(_), path, _)) => Some(Identifier {
                name: String::from(format!("!{}", path.segments[0].ident.clone())),
            }),
            Some((None, path, _)) => Some(Identifier::from(path.segments[0].ident.clone())),
        };

        if let syn::Type::Path(syn::TypePath { path, .. }) = *item_impl.self_ty {
            Self {
                attributes: Attributes {
                    attributes: item_impl
                        .attrs
                        .into_iter()
                        .map(|x| Attribute::from(x.parse_meta().expect("Failed to parse Meta")))
                        .collect(),
                },
                trait_,
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
                trait_: None,
                self_: Identifier {
                    name: String::from("Test")
                },
                items: None
            }
        );
    }

    #[test]
    fn impl_block_trait() {
        assert_eq!(
            Impl::from(parse::<ItemImpl>(quote! {impl Trait for Test {}})),
            Impl {
                attributes: Attributes { attributes: vec![] },
                trait_: Some(Identifier {
                    name: String::from("Trait")
                }),
                self_: Identifier {
                    name: String::from("Test")
                },
                items: None
            }
        );
    }

    #[test]
    fn impl_block_trait_bang() {
        assert_eq!(
            Impl::from(parse::<ItemImpl>(quote! {impl !Trait for Test {}})),
            Impl {
                attributes: Attributes { attributes: vec![] },
                trait_: Some(Identifier {
                    name: String::from("!Trait")
                }),
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
                trait_: None,
                self_: Identifier {
                    name: String::from("Test")
                },
                items: None
            }
        );
    }
}
