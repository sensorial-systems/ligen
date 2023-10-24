pub use crate::*;

pub fn constant() -> Object {
    Object {
        mutability: Mutability::Constant,
        identifier: "CONSTANT".into(),
        type_: Primitive::Boolean.into(),
        literal: Literal::Boolean(false)
    }
}