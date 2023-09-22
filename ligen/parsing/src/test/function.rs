// TODO: Move to ligen_ir::function::mock?

use ligen_ir::*;

pub fn function() -> Function {
    Function {
        attributes: Attributes { attributes: vec![] },
        visibility: Visibility::Private,
        synchrony: Synchrony::Synchronous,
        path: Identifier::new("test").into(),
        inputs: vec![],
        output: None
    }
}

pub fn function_input() -> Function {
    Function {
        attributes: Attributes { attributes: vec![] },
        visibility: Visibility::Private,
        synchrony: Synchrony::Synchronous,
        path: Identifier::new("test").into(),
        inputs: vec![
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("a"),
                type_: Type::Composite(Identifier::new("String").into(), Default::default())
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("b"),
                type_: Type::Composite(Identifier::new("String").into(), Default::default())
            },
        ],
        output: None
    }
}

pub fn function_output() -> Function {
    Function {
        attributes: Attributes { attributes: vec![] },
        visibility: Visibility::Private,
        synchrony: Synchrony::Synchronous,
        path: Identifier::new("test").into(),
        inputs: vec![],
        output: Some(Type::Composite(Identifier::new("String").into(), Default::default()))
    }
}

pub fn function_input_output() -> Function {
    Function {
        attributes: Attributes { attributes: vec![] },
        visibility: Visibility::Private,
        synchrony: Synchrony::Synchronous,
        path: Identifier::new("test").into(),
        inputs: vec![
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("a"),
                type_: Type::Composite(Identifier::new("String").into(), Default::default())
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("b"),
                type_: Type::Reference(Reference {
                    mutability: Mutability::Constant,
                    type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                })
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("c"),
                type_: Type::Reference(Reference {
                    mutability: Mutability::Mutable,
                    type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                })
            },
        ],
        output: Some(Type::Reference(Reference {
            mutability: Mutability::Constant,
            type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
        }))
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
        visibility: Visibility::Private,
        synchrony: Synchrony::Synchronous,
        path: Identifier::new("test").into(),
        inputs: vec![],
        output: None
    }
}

pub fn function_async() -> Function {
    Function {
        attributes: Attributes { attributes: vec![] },
        visibility: Visibility::Private,
        synchrony: Synchrony::Asynchronous,
        path: Identifier::new("test").into(),
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
        visibility: Visibility::Private,
        synchrony: Synchrony::Asynchronous,
        path: Identifier::new("test").into(),
        inputs: vec![
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("a"),
                type_: Type::Composite(Identifier::new("String").into(), Default::default())
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("b"),
                type_: Type::Reference(Reference {
                    mutability: Mutability::Constant,
                    type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                })
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("c"),
                type_: Type::Reference(Reference {
                    mutability: Mutability::Mutable,
                    type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                })
            },
        ],
        output: Some(Type::Reference(Reference {
            mutability: Mutability::Constant,
            type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
        }))
    }
}

pub fn function_pub() -> Function {
    Function {
        attributes: Attributes { attributes: vec![] },
        visibility: Visibility::Public,
        synchrony: Synchrony::Synchronous,
        path: Identifier::new("test").into(),
        inputs: vec![],
        output: None
    }
}