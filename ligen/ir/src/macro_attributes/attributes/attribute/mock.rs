use crate::*;

pub fn attribute_group() -> Attribute {
    Attribute::Group(
        Identifier::new("c"),
        Attributes {
            attributes: vec![
                Attribute::Named(
                    Identifier::new("int"),
                    Literal::String(String::from("sized"))
                )
            ]
        }
    )
}

pub fn attribute_named() -> Attribute {
    Attribute::Named(
        Identifier::new("int"),
        Literal::String(
            String::from("sized")
        )
    )
}

pub fn attribute_literal() -> Attribute {
    Attribute::Literal(
        Literal::String(
            String::from("c")
        )
    )
}