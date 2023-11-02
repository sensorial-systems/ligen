use crate::*;

pub fn primitive_parameter() -> Parameter {
    Parameter {
        identifier: Identifier::new("integer"),
        type_: Type::i32(),
        .. Default::default()
    }
}

pub fn parameter_attribute() -> Parameter {
    Parameter {
        attributes: Attribute::Group("attribute".into(), Default::default()).into(),
        identifier: Identifier::new("integer"),
        type_: Type::i32()
    }
}

pub fn composite_parameter() -> Parameter {
    Parameter {
        identifier: Identifier::new("name"),
        type_: Type::string(),
        .. Default::default()
    }
}

pub fn constant_reference_parameter() -> Parameter {
    Parameter {
        identifier: Identifier::new("name"),
        type_: Type::Reference(
            Reference {
                mutability: Mutability::Constant,
                type_: Type::string().into()
            }
        ),
        .. Default::default()
    }
}

pub fn mutable_reference_parameter() -> Parameter {
    Parameter {
        identifier: Identifier::new("name"),
        type_: Type::Reference(
            Reference {
                mutability: Mutability::Mutable,
                type_: Type::string().into()
            }
        ),
        .. Default::default()
    }
}

pub fn receiver_parameter() -> Parameter {
    Parameter {
        identifier: Identifier::new("self"),
        type_: Type::from("Self"),
        .. Default::default()
    }
}

pub fn reference_receiver_parameter() -> Parameter {
    Parameter {
        identifier: Identifier::new("self"),
        type_: Type::Reference(
            Reference {
                mutability: Mutability::Constant,
                type_: Box::new(Type::from("Self"))            }
        ),
        .. Default::default()
    }
}

pub fn mutable_receiver_parameter() -> Parameter {
    Parameter {
        identifier: Identifier::new("self"),
        type_: Type::Reference(
            Reference {
                mutability: Mutability::Mutable,
                type_: Box::new(Type::from("Self"))
            }
        ),
        .. Default::default()
    }
}