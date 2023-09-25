use crate::*;

pub fn enumeration() -> Enumeration {
    Enumeration {
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
        ]
    }
}