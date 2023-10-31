use crate::*;

pub fn function() -> Function {
    Function {
        attributes: Attributes { attributes: vec![] },
        visibility: Visibility::Public,
        synchrony: Synchrony::Synchronous,
        identifier: "test".into(),
        inputs: vec![],
        output: None
    }
}

pub fn function_input() -> Function {
    Function {
        attributes: Attributes { attributes: vec![] },
        visibility: Visibility::Public,
        synchrony: Synchrony::Synchronous,
        identifier: "test".into(),
        inputs: vec![
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("a"),
                type_: Integer::I32.into()
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("b"),
                type_: Integer::I32.into()
            },
        ],
        output: None
    }
}

pub fn function_output() -> Function {
    Function {
        attributes: Attributes { attributes: vec![] },
        visibility: Visibility::Public,
        synchrony: Synchrony::Synchronous,
        identifier: "test".into(),
        inputs: vec![],
        output: Some(Type::Composite(Identifier::new("String").into()))
    }
}

pub fn function_input_output() -> Function {
    Function {
        attributes: Attributes { attributes: vec![] },
        visibility: Visibility::Public,
        synchrony: Synchrony::Synchronous,
        identifier: "test".into(),
        inputs: vec![
            Parameter {
                identifier: Identifier::new("a"),
                type_: Integer::I32.into(),
                .. Default::default()
            },
            Parameter {
                identifier: Identifier::new("b"),
                type_: Integer::I32.into(),
                .. Default::default()
            }
        ],
        output: Some(Integer::I32.into())
    }
}

pub fn function_attribute() -> Function {
    Function {
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
        visibility: Visibility::Public,
        synchrony: Synchrony::Synchronous,
        identifier: "test".into(),
        inputs: vec![],
        output: None
    }
}

pub fn function_async() -> Function {
    Function {
        attributes: Attributes { attributes: vec![] },
        visibility: Visibility::Public,
        synchrony: Synchrony::Asynchronous,
        identifier: "test".into(),
        inputs: vec![],
        output: None
    }
}

pub fn function_complete() -> Function {
    Function {
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
        visibility: Visibility::Public,
        synchrony: Synchrony::Asynchronous,
        identifier: "test".into(),
        inputs: vec![
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("a"),
                type_: Type::Composite(Identifier::new("String").into())
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("b"),
                type_: Type::Reference(Reference {
                    mutability: Mutability::Constant,
                    type_: Box::new(Type::Composite(Identifier::new("String").into()))
                })
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("c"),
                type_: Type::Reference(Reference {
                    mutability: Mutability::Mutable,
                    type_: Box::new(Type::Composite(Identifier::new("String").into()))
                })
            },
        ],
        output: Some(Type::Reference(Reference {
            mutability: Mutability::Constant,
            type_: Box::new(Type::Composite(Identifier::new("String").into()))
        }))
    }
}