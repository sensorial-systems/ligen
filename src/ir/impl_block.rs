use crate::ir::{Attribute, Attributes, Constant, Function, Identifier};
use syn::ItemImpl;

#[derive(Debug, PartialEq)]
/// Function Struct
pub struct Impl {
    /// attributes field
    pub attributes: Attributes,
    /// self_ field
    pub self_: Identifier,
    /// items field
    pub items: Vec<ImplItem>,
}

#[derive(Debug, PartialEq)]
/// ImplItem Enum
pub enum ImplItem {
    /// Constant variant
    Constant(Constant),
    /// Method variant
    Method(Function),
}

impl From<syn::ImplItem> for ImplItem {
    fn from(impl_item: syn::ImplItem) -> Self {
        match impl_item {
            syn::ImplItem::Const(impl_item_const) => {
                Self::Constant(Constant::from(impl_item_const))
            }
            syn::ImplItem::Method(impl_item_method) => {
                Self::Method(Function::from(impl_item_method))
            }
            _ => panic!("Only Const and Method Impl items are currently supported"),
        }
    }
}

impl From<ItemImpl> for Impl {
    fn from(item_impl: ItemImpl) -> Self {
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
                items: item_impl
                    .items
                    .into_iter()
                    .map(|x| ImplItem::from(x))
                    .collect(),
            }
        } else {
            panic!("Impl Block Identifier not found")
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Attribute, Attributes, Constant, Function, Identifier, Impl, ImplItem, ItemImpl};
    use crate::ir::{Atomic, Integer, Literal, Type};
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
                items: vec![]
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
                items: vec![]
            }
        );
    }

    #[test]
    fn impl_block_items_const() {
        assert_eq!(
            Impl::from(parse::<ItemImpl>(quote! {
                impl Test {
                    const a: i32 = 2;
                }
            })),
            Impl {
                attributes: Attributes { attributes: vec![] },
                self_: Identifier {
                    name: String::from("Test")
                },
                items: vec![ImplItem::Constant(Constant {
                    identifier: Identifier {
                        name: String::from("a")
                    },
                    type_: Type::Atomic(Atomic::Integer(Integer::I32)),
                    literal: Literal::Integer(2)
                })]
            }
        );
    }

    #[test]
    fn impl_block_items_method() {
        assert_eq!(
            Impl::from(parse::<ItemImpl>(quote! {
                impl Test {
                    fn a(){}
                }
            })),
            Impl {
                attributes: Attributes { attributes: vec![] },
                self_: Identifier {
                    name: String::from("Test")
                },
                items: vec![ImplItem::Method(Function {
                    attributes: Attributes { attributes: vec![] },
                    asyncness: None,
                    identifier: Identifier {
                        name: String::from("a")
                    },
                    input: vec![],
                    output: None
                })]
            }
        );
    }

    #[test]
    fn impl_block_items() {
        assert_eq!(
            Impl::from(parse::<ItemImpl>(quote! {
                impl Test {
                    const a: i32 = 2;
                    fn b(){}
                }
            })),
            Impl {
                attributes: Attributes { attributes: vec![] },
                self_: Identifier {
                    name: String::from("Test")
                },
                items: vec![
                    ImplItem::Constant(Constant {
                        identifier: Identifier {
                            name: String::from("a")
                        },
                        type_: Type::Atomic(Atomic::Integer(Integer::I32)),
                        literal: Literal::Integer(2)
                    }),
                    ImplItem::Method(Function {
                        attributes: Attributes { attributes: vec![] },
                        asyncness: None,
                        identifier: Identifier {
                            name: String::from("b")
                        },
                        input: vec![],
                        output: None
                    })
                ]
            }
        );
    }
}
