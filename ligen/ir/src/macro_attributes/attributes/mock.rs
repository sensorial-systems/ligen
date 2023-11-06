use crate::{*, macro_attributes::{Group, Named}};

pub fn parse_attributes() -> Attributes {
    Group::new(
        "c",
        Named::new("int", "sized")
    ).into()
}

pub fn parse_literals() -> Attributes {
    Group::new(
        "c",
        vec![
            Attribute::Group(Group::new(
                "marshal_as",
                vec![
                    Named::new("name", "hello"),
                    Named::new("uuid", 5)
                ]
            )),
            Attribute::Named(Named::new("int", "sized"))
        ]
    ).into()
}
