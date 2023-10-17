pub use crate::*;

pub fn constant() -> Constant {
    Constant {
        identifier: "CONSTANT".into(),
        type_: Primitive::Boolean.into(),
        literal: Literal::Boolean(false)
    }
}