use crate::*;

pub fn structure() -> TypeDefinition {
    TypeDefinition {
        identifier: "Structure".into(),
        definition: Structure {
            fields: vec! [
                Field {
                    attributes: Default::default(),
                    visibility: Visibility::Private,
                    identifier: Some("integer".into()),
                    type_: Type::Primitive(Primitive::Integer(Integer::I32))
                }
            ]
        }.into(),
        ..Default::default()
    }
}