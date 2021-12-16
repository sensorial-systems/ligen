mod implementation_item;
pub use implementation_item::*;

use crate::prelude::*;
use ligen_ir::{Attributes, Identifier, Type, Implementation};

impl TryFrom<TokenStream> for Implementation {
    type Error = Error;
    fn try_from(tokenstream: TokenStream) -> Result<Self> {
        syn::parse2::<syn::ItemImpl>(tokenstream)
            .map_err(|_| "Failed to parse to Implementation.".into())
            .and_then(|item| item.try_into())
    }
}

impl TryFrom<proc_macro::TokenStream> for Implementation {
    type Error = Error;
    fn try_from(tokenstream: proc_macro::TokenStream) -> Result<Self> {
        let tokenstream: TokenStream = tokenstream.into();
        tokenstream.try_into()
    }
}

impl TryFrom<syn::ItemImpl> for Implementation {
    type Error = Error;
    fn try_from(item_impl: syn::ItemImpl) -> Result<Self> {
        if let syn::Type::Path(syn::TypePath { path, .. }) = *item_impl.self_ty {
            Ok(Self {
                attributes: Attributes {
                    attributes: item_impl
                        .attrs
                        .into_iter()
                        .map(|x| x.parse_meta().expect("Failed to parse Meta").into())
                        .collect(),
                },
                self_: path.into(),
                items: item_impl
                    .items
                    .into_iter()
                    .map(|x| x.try_into().expect("Failed to convert from ImplItem"))
                    .collect(),
            })
        } else {
            Err("Impl Block Identifier not found".into())
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use super::*;
    use crate::{Atomic, Attribute, Integer, Literal, Reference, ReferenceKind, Type, Visibility, Constant, Function, Generics};
    use pretty_assertions::assert_eq;

    #[test]
    fn impl_block() {
        assert_eq!(
            Implementation::try_from(quote! {impl Test {}})
                .expect("Failed to convert from ItemImpl"),
            Implementation {
                attributes: Attributes { attributes: vec![] },
                self_: Type::Compound(Identifier::new("Test").into(), Default::default()),
                items: vec![]
            }
        );
    }

    #[test]
    fn impl_block_attributes() {
        assert_eq!(
            Implementation::try_from(quote! {
                #[test(a = "b")]
                impl Test {}
            }).expect("Failed to convert from ItemImpl"),
            Implementation {
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
                self_: Type::Compound(Identifier::new("Test").into(), Default::default()),
                items: vec![]
            }
        );
    }

    #[test]
    fn impl_block_items_const() {
        assert_eq!(
            Implementation::try_from(quote! {
                impl Test {
                    const a: i32 = 2;
                }
            }).expect("Failed to convert from ItemImpl"),
            Implementation {
                attributes: Attributes { attributes: vec![] },
                self_: Type::Compound(Identifier::new("Test").into(), Default::default()),
                items: vec![ImplementationItem::Constant(Constant {
                    identifier: Identifier::new("a"),
                    type_: Type::Atomic(Atomic::Integer(Integer::I32)),
                    literal: Literal::Integer(2)
                })]
            }
        );
    }

    #[test]
    fn impl_block_items_method() {
        assert_eq!(
            Implementation::try_from(quote! {
                impl Test {
                    fn a(){}
                }
            }).expect("Failed to convert from ItemImpl"),
            Implementation {
                attributes: Attributes { attributes: vec![] },
                self_: Type::Compound(Identifier::new("Test").into(), Default::default()),
                items: vec![ImplementationItem::Method(Function {
                    attributes: Attributes { attributes: vec![] },
                    visibility: Visibility::Inherited,
                    asyncness: None,
                    identifier: Identifier::new("a"),
                    inputs: vec![],
                    output: None
                })]
            }
        );
    }

    #[test]
    fn impl_block_items() {
        assert_eq!(
            Implementation::try_from(quote! {
                impl Test {
                    const a: i32 = 2;
                    fn b(){}
                }
            })
                .expect("Failed to convert from ItemImpl"),
            Implementation {
                attributes: Attributes { attributes: vec![] },
                self_: Type::Compound(Identifier::new("Test").into(), Default::default()),
                items: vec![
                    ImplementationItem::Constant(Constant {
                        identifier: Identifier::new("a"),
                        type_: Type::Atomic(Atomic::Integer(Integer::I32)),
                        literal: Literal::Integer(2)
                    }),
                    ImplementationItem::Method(Function {
                        attributes: Attributes { attributes: vec![] },
                        visibility: Visibility::Inherited,
                        asyncness: None,
                        identifier: Identifier::new("b"),
                        inputs: vec![],
                        output: None
                    })
                ]
            }
        );
    }

    #[test]
    fn impl_block_dependencies() {
        assert_eq!(
            Implementation::try_from(quote! {
                impl Person {
                    pub fn new(name: FullName, age: Age) -> Self { ... }
                    pub fn more_deps(age: Age, a: A, b: B, c: C) -> D;
                    pub fn builtin(&self, age: i32, name: String, name_str: &str, vec: Vec<String>) -> Box<String>;
                }
            })
                .expect("Failed to build implementation from TokenStream")
                .dependencies(),
            vec![
                Type::Compound(Identifier::new("FullName").into(), Default::default()),
                Type::Compound(Identifier::new("Age").into(), Default::default()),
                Type::Compound(Identifier::new("A").into(), Default::default()),
                Type::Compound(Identifier::new("B").into(), Default::default()),
                Type::Compound(Identifier::new("C").into(), Default::default()),
                Type::Compound(Identifier::new("D").into(), Default::default()),
                Type::Reference(Reference {kind: ReferenceKind::Borrow, is_constant: true, type_: Box::new(Type::Compound(Identifier::new("Self").into(), Default::default()))}),
                Type::Atomic(Atomic::Integer(Integer::I32)),
                Type::Compound(Identifier::new("String").into(), Default::default()),
                Type::Reference(Reference {kind: ReferenceKind::Borrow, is_constant: true, type_: Box::new(Type::Compound(Identifier::new("str").into(), Default::default()))}),
                Type::Compound(Identifier::new("Vec").into(), Generics { types: vec![ Type::Compound("String".into(), Default::default())]}),
                Type::Compound(Identifier::new("Box").into(), Generics { types: vec![ Type::Compound("String".into(), Default::default())]}),
            ]
        );
    }
}
