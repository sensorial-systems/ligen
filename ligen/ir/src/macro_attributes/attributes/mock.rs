use crate::*;

pub fn parse_attributes() -> Attributes {
    Attributes {
        attributes: vec![
            Attribute::Group(
                Identifier::new("c"),
                Attributes {
                    attributes: vec![Attribute::Named(
                        Identifier::new("int"),
                        Literal::String(String::from("sized"))
                        )
                    ]
                }
            )
        ]
    }
}

pub fn parse_literals() -> Attributes {
    Attributes {
        attributes: vec![
            Attribute::Group(
                Identifier::new("c"),
                Attributes {
                    attributes: vec![
                        Attribute::Group(
                            Identifier::new("marshal_as"),
                            Attributes {
                                attributes: vec![
                                    Attribute::Named(
                                        Identifier::new("name"),
                                        Literal::String(String::from("hello"))
                                    ),
                                    Attribute::Named(
                                        Identifier::new("uuid"),
                                        Literal::Integer(5)
                                    )
                                ]
                            }
                        ),
                        Attribute::Named(
                            Identifier::new("int"),
                            Literal::String(String::from("sized"))
                        )
                    ]
                }
            )
        ]
    }
}
