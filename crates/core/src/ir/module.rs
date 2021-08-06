//! Module representation.

use crate::prelude::*;
use crate::ir::{Object, Path, Structure, Implementation};
use std::convert::TryFrom;
use std::collections::HashMap;
use itertools::Itertools;

/// Module representation.
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    objects: Vec<Object>
}

impl TryFrom<syn::File> for Module {
    type Error = Error;
    fn try_from(file: syn::File) -> Result<Self> {
        let mut objects: HashMap<Path, (Option<Structure>, Vec<Implementation>)> = HashMap::new();
        for item in file.items {
            match item {
                syn::Item::Struct(structure) => {
                    let structure = Structure::try_from(structure)?;
                    let path = structure.identifier.clone().into();
                    if let Some((optional_structure, _implementations)) = objects.get_mut(&path) {
                        *optional_structure = Some(structure);
                    } else {
                        objects.insert(path, (Some(structure), Default::default()));
                    }
                },
                syn::Item::Impl(implementation) => {
                    let implementation = Implementation::try_from(implementation)?;
                    let path = implementation.self_.path();
                    if let Some((_structure, implementations)) = objects.get_mut(&path) {
                        implementations.push(implementation);
                    } else {
                        objects.insert(path, (None, vec![implementation]));
                    }
                }
                _ => ()
            }
        }
        let objects = objects
            .into_iter()
            .map(|(path, (structure, implementations))| Object {
                path,
                structure,
                implementations
            })
            .sorted_by(|a, b| a.path.cmp(&b.path))
            .collect();
        Ok(Self { objects })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Object, Atomic, Integer, Type, Visibility, Function, Structure, Parameter, Implementation, ImplementationItem, Field};
    use quote::quote;
    use syn::parse_quote::parse;

    #[test]
    fn object() {
        assert_eq!(
            Module::try_from(parse::<syn::File>(quote! {
                pub struct Object {
                    pub integer: i32
                }

                impl Object {
                    pub fn new(integer: i32) -> Self {
                        Self { integer }
                    }
                }

                pub struct AnotherObject;
            })).expect("Failed to convert from ItemImpl"),
            Module {
                objects: vec![
                    Object {
                        path: "AnotherObject".into(),
                        structure: Some(Structure {
                            attributes: Default::default(),
                            visibility: Visibility::Public,
                            identifier: "AnotherObject".into(),
                            fields: Default::default(),
                        }),
                        implementations: Default::default()
                    },
                    Object {
                        path: "Object".into(),
                        structure: Some(Structure {
                            attributes: Default::default(),
                            visibility: Visibility::Public,
                            identifier: "Object".into(),
                            fields: vec![
                                Field {
                                    attributes: Default::default(),
                                    visibility: Visibility::Public,
                                    identifier: "integer".into(),
                                    type_: Type::Atomic(Atomic::Integer(Integer::I32))
                                }
                            ]
                        }),
                        implementations: vec![
                            Implementation {
                                attributes: Default::default(),
                                self_: Type::Compound("Object".into()),
                                items: vec![
                                    ImplementationItem::Method(Function {
                                        attributes: Default::default(),
                                        visibility: Visibility::Public,
                                        asyncness: None,
                                        identifier: "new".into(),
                                        inputs: vec![
                                            Parameter {
                                                identifier: "integer".into(),
                                                type_: Type::Atomic(Atomic::Integer(Integer::I32))
                                            }
                                        ],
                                        output: Some(Type::Compound("Self".into()))
                                    }
                                    )
                                ]
                            }
                        ]
                    }
                ]
            }
        );
    }
}