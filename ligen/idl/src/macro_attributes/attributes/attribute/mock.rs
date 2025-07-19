use crate::{*, macro_attributes::{Named, Group}};

pub fn attribute_group() -> Attribute {
    Group::new("c", Named::new("int", "sized")).into()
}

pub fn attribute_empty_group() -> Attribute {
    Group::from("c").into()
}

pub fn attribute_named() -> Attribute {
    Named::new("int", "sized").into()
}

pub fn attribute_literal() -> Attribute {
    Attribute::Literal(
        Literal::String(
            String::from("c")
        )
    )
}