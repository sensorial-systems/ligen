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
                type_: Type::i32()
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("b"),
                type_: Type::i32()
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
        output: Some(Type::string())
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
                type_: Type::i32(),
                .. Default::default()
            },
            Parameter {
                identifier: Identifier::new("b"),
                type_: Type::i32(),
                .. Default::default()
            }
        ],
        output: Some(Type::i32())
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
                type_: Type::string()
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("b"),
                type_: Type::Reference(Reference {
                    mutability: Mutability::Constant,
                    type_: Box::new(Type::string())
                })
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("c"),
                type_: Type::Reference(Reference {
                    mutability: Mutability::Mutable,
                    type_: Box::new(Type::string())
                })
            },
        ],
        output: Some(Type::Reference(Reference {
            mutability: Mutability::Constant,
            type_: Box::new(Type::string())
        }))
    }
}