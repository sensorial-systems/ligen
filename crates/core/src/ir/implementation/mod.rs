mod implementation_item;
pub use implementation_item::*;

use crate::prelude::*;
use crate::ir::{Attributes, Identifier, Type};
use crate::r#macro;
use proc_macro2::TokenStream;
use std::convert::{TryFrom, TryInto};
use syn::{parse2, ItemImpl};
use crate::ir::processing::ReplaceIdentifier;

#[derive(Debug, PartialEq, Clone)]
/// Function Struct
pub struct Implementation {
    /// Attributes field.
    pub attributes: Attributes,
    /// Self field.
    pub self_: Type,
    /// Items field.
    pub items: Vec<ImplementationItem>,
}

impl TryFrom<TokenStream> for Implementation {
    type Error = Error;
    fn try_from(tokenstream: TokenStream) -> Result<Self> {
        parse2::<ItemImpl>(tokenstream)
            .map_err(|_| "Failed to parse to Implementation.".into())
            .and_then(|item| item.try_into())
    }
}

impl TryFrom<r#macro::TokenStream> for Implementation {
    type Error = Error;
    fn try_from(tokenstream: r#macro::TokenStream) -> Result<Self> {
        let tokenstream: TokenStream = tokenstream.into();
        tokenstream.try_into()
    }
}

impl TryFrom<ItemImpl> for Implementation {
    type Error = Error;
    fn try_from(item_impl: ItemImpl) -> Result<Self> {
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

impl Implementation {
    /// Maps the dependencies in the method signatures.
    pub fn dependencies(&self) -> Vec<Type> {
        let mut deps: Vec<Type> = vec![];
        for item in &self.items {
            if let ImplementationItem::Method(method) = item {
                method.inputs.clone().into_iter().for_each(|parameter| {
                    if !deps.iter().any(|typ| typ == &parameter.type_) {
                        deps.push(parameter.type_);
                    }
                });
                if let Some(type_) = method.output.clone() {
                    if !deps.iter().any(|typ| typ == &type_)
                        && type_ != Type::Compound(Identifier::new("Self").into())
                    {
                        deps.push(type_);
                    }
                }
            }
        }
        deps
    }

    /// Replace all the occurrences of `Self` by the real object name.
    /// e.g.:
    /// ```rust,compile_fail
    /// impl Object {
    ///     fn f(self: &Self) {}
    /// }
    /// ```
    /// becomes
    /// ```rust,compile_fail
    /// impl Object {
    ///     fn f(self: &Object) {}
    /// }
    /// ```
    pub fn replace_self_with_explicit_names(&mut self) {
        let identifier = self.self_.path().last();
        let mut lower_case_identifier = identifier.clone();
        lower_case_identifier.name = lower_case_identifier.name.to_lowercase();
        self.replace_identifier(&Identifier::from("Self"), &identifier);
    }
}



#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use super::*;
    use crate::ir::{Atomic, Attribute, Integer, Literal, Reference, ReferenceKind, Type, Visibility, Constant, Function};
    use quote::quote;
    use syn::parse_quote::parse;

    #[test]
    fn impl_block() {
        assert_eq!(
            Implementation::try_from(parse::<ItemImpl>(quote! {impl Test {}}))
                .expect("Failed to convert from ItemImpl"),
            Implementation {
                attributes: Attributes { attributes: vec![] },
                self_: Type::Compound(Identifier::new("Test").into()),
                items: vec![]
            }
        );
    }

    #[test]
    fn impl_block_attributes() {
        assert_eq!(
            Implementation::try_from(parse::<ItemImpl>(quote! {
                #[test(a = "b")]
                impl Test {}
            }))
                .expect("Failed to convert from ItemImpl"),
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
                self_: Type::Compound(Identifier::new("Test").into()),
                items: vec![]
            }
        );
    }

    #[test]
    fn impl_block_items_const() {
        assert_eq!(
            Implementation::try_from(parse::<ItemImpl>(quote! {
                impl Test {
                    const a: i32 = 2;
                }
            }))
                .expect("Failed to convert from ItemImpl"),
            Implementation {
                attributes: Attributes { attributes: vec![] },
                self_: Type::Compound(Identifier::new("Test").into()),
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
            Implementation::try_from(parse::<ItemImpl>(quote! {
                impl Test {
                    fn a(){}
                }
            }))
                .expect("Failed to convert from ItemImpl"),
            Implementation {
                attributes: Attributes { attributes: vec![] },
                self_: Type::Compound(Identifier::new("Test").into()),
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
            Implementation::try_from(parse::<ItemImpl>(quote! {
                impl Test {
                    const a: i32 = 2;
                    fn b(){}
                }
            }))
                .expect("Failed to convert from ItemImpl"),
            Implementation {
                attributes: Attributes { attributes: vec![] },
                self_: Type::Compound(Identifier::new("Test").into()),
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
            Implementation::try_from(parse::<ItemImpl>(quote! {
                impl Person {
                    pub fn new(name: FullName, age: Age) -> Self { ... }
                    pub fn more_deps(age: Age, a: A, b: B, c: C) -> D;
                    pub fn builtin(&self, age: i32, name: String, name_str: &str, vec: Vec<String>) -> Box<Rc<Mutex<Arc<HashMap<String, Option<Result<String, Error>>>>>>>;
                }
            }))
                .expect("Failed to build implementation from TokenStream")
                .dependencies(),
            vec![
                Type::Compound(Identifier::new("FullName").into()),
                Type::Compound(Identifier::new("Age").into()),
                Type::Compound(Identifier::new("A").into()),
                Type::Compound(Identifier::new("B").into()),
                Type::Compound(Identifier::new("C").into()),
                Type::Compound(Identifier::new("D").into()),
                Type::Reference(Reference {kind: ReferenceKind::Borrow, is_constant: true, type_: Box::new(Type::Compound(Identifier::new("Self").into()))}),
                Type::Atomic(Atomic::Integer(Integer::I32)),
                Type::Compound(Identifier::new("String").into()),
                Type::Reference(Reference {kind: ReferenceKind::Borrow, is_constant: true, type_: Box::new(Type::Compound(Identifier::new("str").into()))}),
                Type::Compound(Identifier::new("Vec").into()),
                Type::Compound(Identifier::new("Box").into()),
            ]
        );
    }
}
