use crate::{Type, TypeAlias, TypeDefinition};

pub fn type_alias() -> TypeDefinition {
    TypeDefinition {
        identifier: "Integer".into(),
        definition: TypeAlias {
            type_: Type::i32()
        }.into(),
        ..Default::default()
    }
}