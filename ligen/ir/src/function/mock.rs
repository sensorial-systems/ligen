use crate::{*, macro_attributes::{Group, Named}};

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
                type_: Type::i32(),
                default_value: Default::default()
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("b"),
                type_: Type::i32(),
                default_value: Default::default()
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
        attributes: Group::new("test", Named::new("a", "b")).into(),
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
        attributes: Group::new("test", Named::new("a", "b")).into(),
        visibility: Visibility::Public,
        synchrony: Synchrony::Asynchronous,
        identifier: "test".into(),
        inputs: vec![
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("a"),
                type_: Type::string(),
                default_value: Default::default()
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("b"),
                type_: Type::constant_reference(Type::string()),
                default_value: Default::default()
            },
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("c"),
                type_: Type::mutable_reference(Type::string()),
                default_value: Default::default()
            },
        ],
        output: Some(Type::constant_reference(Type::string()))
    }
}