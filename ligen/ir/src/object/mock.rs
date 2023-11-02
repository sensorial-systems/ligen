pub use crate::*;

pub fn constant() -> Object {
    Object {
        mutability: Mutability::Constant,
        identifier: "CONSTANT".into(),
        type_: Type::boolean(),
        literal: false.into()
    }
}