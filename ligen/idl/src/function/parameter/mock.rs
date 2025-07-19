use crate::{*, macro_attributes::Group};

pub fn primitive_parameter() -> Parameter {
    Parameter {
        identifier: Identifier::new("integer"),
        type_: Type::i32(),
        .. Default::default()
    }
}

pub fn parameter_attribute() -> Parameter {
    Parameter {
        attributes: Group::from("attribute").into(),
        identifier: Identifier::new("integer"),
        type_: Type::i32(),
        default_value: Default::default()
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
        type_: Type::constant_reference(Type::string()),
        .. Default::default()
    }
}

pub fn mutable_reference_parameter() -> Parameter {
    Parameter {
        identifier: Identifier::new("name"),
        type_: Type::mutable_reference(Type::string()),
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
        type_: Type::constant_reference("Self"),
        .. Default::default()
    }
}

pub fn mutable_receiver_parameter() -> Parameter {
    Parameter {
        identifier: Identifier::new("self"),
        type_: Type::mutable_reference("Self"),
        .. Default::default()
    }
}