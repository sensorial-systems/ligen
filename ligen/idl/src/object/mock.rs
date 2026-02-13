pub use crate::*;

pub fn constant() -> Object {
    Object {
        visibility: Visibility::Private,
        mutability: Mutability::Constant,
        identifier: "CONSTANT".into(),
        type_: Type::boolean(),
        literal: false.into(),
    }
}
