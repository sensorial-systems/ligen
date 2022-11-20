pub mod implementation_item;

use crate::prelude::*;
use crate::{Attributes, Implementation, Type};
use crate::parsing::implementation::implementation_item::TypeImplItem;

impl TryFrom<ProcMacro2TokenStream> for Implementation {
    type Error = Error;
    fn try_from(tokenstream: ProcMacro2TokenStream) -> Result<Self> {
        syn::parse2::<syn::ItemImpl>(tokenstream.into())
            .map_err(|_| "Failed to parse to Implementation.".into())
            .and_then(|item| SynItemImpl::from(item).try_into())
    }
}

impl TryFrom<ProcMacroTokenStream> for Implementation {
    type Error = Error;
    fn try_from(ProcMacroTokenStream(tokenstream): ProcMacroTokenStream) -> Result<Self> {
        let tokenstream: TokenStream = tokenstream.into();
        ProcMacro2TokenStream::from(tokenstream).try_into()
    }
}

impl TryFrom<SynItemImpl> for Implementation {
    type Error = Error;
    fn try_from(SynItemImpl(item_impl): SynItemImpl) -> Result<Self> {
        if let syn::Type::Path(syn::TypePath { path, .. }) = *item_impl.self_ty {
            let self_ = Type::from(SynPath::from(path));
            Ok(Self {
                attributes: Attributes {
                    attributes: item_impl
                        .attrs
                        .into_iter()
                        .map(|x| SynMeta::from(x.parse_meta().expect("Failed to parse Meta")).into())
                        .collect(),
                },
                self_: self_.clone(),
                items: item_impl
                    .items
                    .into_iter()
                    .map(|x| TypeImplItem(self_.clone(), x).try_into().expect("Failed to convert from ImplItem"))
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
    use crate::{Primitive, Attribute, Integer, Literal, Reference, Type, Visibility, Constant, Function, Generics, Mutability, Method, ImplementationItem, Identifier};
    use pretty_assertions::assert_eq;

    #[test]
    fn impl_block() {
        assert_eq!(
            Implementation::try_from(ProcMacro2TokenStream(quote! {impl Test {}}))
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
            Implementation::try_from(ProcMacro2TokenStream(quote! {
                #[test(a = "b")]
                impl Test {}
            })).expect("Failed to convert from ItemImpl"),
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
            Implementation::try_from(ProcMacro2TokenStream(quote! {
                impl Test {
                    const a: i32 = 2;
                }
            })).expect("Failed to convert from ItemImpl"),
            Implementation {
                attributes: Attributes { attributes: vec![] },
                self_: Type::Compound(Identifier::new("Test").into(), Default::default()),
                items: vec![ImplementationItem::Constant(Constant {
                    identifier: Identifier::new("a"),
                    type_: Type::Primitive(Primitive::Integer(Integer::I32)),
                    literal: Literal::Integer(2)
                })]
            }
        );
    }

    #[test]
    fn impl_block_items_method() {
        assert_eq!(
            Implementation::try_from(ProcMacro2TokenStream(quote! {
                impl Test {
                    fn a(){}
                }
            })).expect("Failed to convert from ItemImpl"),
            Implementation {
                attributes: Attributes { attributes: vec![] },
                self_: Type::Compound(Identifier::new("Test").into(), Default::default()),
                items: vec![ImplementationItem::Method(Function {
                    attributes: Attributes { attributes: vec![] },
                    method: Some(Method {
                        mutability: Mutability::Constant,
                        owner: Type::Compound(Identifier::new("Test").into(), Default::default())
                    }),
                    visibility: Visibility::Private,
                    asyncness: None,
                    path: Identifier::new("a").into(),
                    inputs: vec![],
                    output: None
                })]
            }
        );
    }

    #[test]
    fn impl_block_items() {
        assert_eq!(
            Implementation::try_from(ProcMacro2TokenStream(quote! {
                impl Test {
                    const a: i32 = 2;
                    fn b(){}
                }
            }))
                .expect("Failed to convert from ItemImpl"),
            Implementation {
                attributes: Attributes { attributes: vec![] },
                self_: Type::Compound(Identifier::new("Test").into(), Default::default()),
                items: vec![
                    ImplementationItem::Constant(Constant {
                        identifier: Identifier::new("a"),
                        type_: Type::Primitive(Primitive::Integer(Integer::I32)),
                        literal: Literal::Integer(2)
                    }),
                    ImplementationItem::Method(Function {
                        attributes: Attributes { attributes: vec![] },
                        method: Some(Method {
                            mutability: Mutability::Constant,
                            owner: Type::Compound(Identifier::new("Test").into(), Default::default())
                        }),
                        visibility: Visibility::Private,
                        asyncness: None,
                        path: Identifier::new("b").into(),
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
            Implementation::try_from(ProcMacro2TokenStream(quote! {
                impl Person {
                    pub fn new(name: FullName, age: Age) -> Self { ... }
                    pub fn more_deps(age: Age, a: A, b: B, c: C) -> D;
                    pub fn builtin(&self, age: i32, name: String, name_str: &str, vec: Vec<String>) -> Box<String>;
                }
            }))
                .expect("Failed to build implementation from TokenStream")
                .dependencies(),
            vec![
                Type::Compound(Identifier::new("FullName").into(), Default::default()),
                Type::Compound(Identifier::new("Age").into(), Default::default()),
                Type::Compound(Identifier::new("A").into(), Default::default()),
                Type::Compound(Identifier::new("B").into(), Default::default()),
                Type::Compound(Identifier::new("C").into(), Default::default()),
                Type::Compound(Identifier::new("D").into(), Default::default()),
                Type::Reference(Reference {mutability: Mutability::Constant, type_: Box::new(Type::Compound(Identifier::new("Self").into(), Default::default()))}),
                Type::Primitive(Primitive::Integer(Integer::I32)),
                Type::Compound(Identifier::new("String").into(), Default::default()),
                Type::Reference(Reference {mutability: Mutability::Constant, type_: Box::new(Type::Compound(Identifier::new("str").into(), Default::default()))}),
                Type::Compound(Identifier::new("Vec").into(), Generics { types: vec![ Type::Compound("String".into(), Default::default())]}),
                Type::Compound(Identifier::new("Box").into(), Generics { types: vec![ Type::Compound("String".into(), Default::default())]}),
            ]
        );
    }
}
