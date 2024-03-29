use crate::*;

pub fn enumeration() -> TypeDefinition {
    TypeDefinition {
        identifier: "Enumeration".into(),
        definition: Enumeration {
            variants: vec! [
                Variant {
                    attributes: Default::default(),
                    identifier: "Integer".into(),
                },
                Variant {
                    attributes: Default::default(),
                    identifier: "Float".into(),
                },
                Variant {
                    attributes: Default::default(),
                    identifier: "Boolean".into()
                }
            ],
        }.into(),
        ..Default::default()
    }
}