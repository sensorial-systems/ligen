use crate::*;

pub fn structure() -> Structure {
    Structure {
        identifier: "Structure".into(),
        fields: vec! [
            Field {
                attributes: Default::default(),
                visibility: Visibility::Private,
                identifier: Some("integer".into()),
                type_: Type::Primitive(Primitive::Integer(Integer::I32))
            }
        ],
        .. Default::default()
    }
}