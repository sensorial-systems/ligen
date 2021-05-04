use crate::ir::{Attributes, Constant, Function, Identifier};
use std::convert::{TryFrom, TryInto};
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

impl TryFrom<syn::ImplItem> for ImplItem {
    type Error = &'static str;
    fn try_from(impl_item: syn::ImplItem) -> Result<Self, Self::Error> {
        match impl_item {
            syn::ImplItem::Const(impl_item_const) => Ok(Self::Constant(impl_item_const.into())),
            syn::ImplItem::Method(impl_item_method) => Ok(Self::Method(impl_item_method.into())),
            _ => Err("Only Const and Method Impl items are currently supported"),
        }
    }
}

impl TryFrom<ItemImpl> for Impl {
    type Error = &'static str;
    fn try_from(item_impl: ItemImpl) -> Result<Self, Self::Error> {
        if let syn::Type::Path(syn::TypePath { path, .. }) = *item_impl.self_ty {
            Ok(Self {
                attributes: Attributes {
                    attributes: item_impl
                        .attrs
                        .into_iter()
                        .map(|x| x.parse_meta().expect("Failed to parse Meta").into())
                        .collect(),
                },
                self_: path.segments[0].ident.clone().into(),
                items: item_impl
                    .items
                    .into_iter()
                    .map(|x| x.try_into().expect("Failed to convert from ImplItem"))
                    .collect(),
            })
        } else {
            Err("Impl Block Identifier not found")
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use super::{Attributes, Constant, Function, Identifier, Impl, ImplItem, ItemImpl};
    use crate::ir::{Atomic, Attribute, Integer, Literal, Type};
    use quote::quote;
    use syn::parse_quote::parse;

    #[test]
    fn impl_block() {
        assert_eq!(
            Impl::try_from(parse::<ItemImpl>(quote! {impl Test {}}))
                .expect("Failed to convert from ItemImpl"),
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
            Impl::try_from(parse::<ItemImpl>(quote! {
                #[test(a = "b")]
                impl Test {}
            }))
            .expect("Failed to convert from ItemImpl"),
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
            Impl::try_from(parse::<ItemImpl>(quote! {
                impl Test {
                    const a: i32 = 2;
                }
            }))
            .expect("Failed to convert from ItemImpl"),
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
            Impl::try_from(parse::<ItemImpl>(quote! {
                impl Test {
                    fn a(){}
                }
            }))
            .expect("Failed to convert from ItemImpl"),
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
            Impl::try_from(parse::<ItemImpl>(quote! {
                impl Test {
                    const a: i32 = 2;
                    fn b(){}
                }
            }))
            .expect("Failed to convert from ItemImpl"),
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