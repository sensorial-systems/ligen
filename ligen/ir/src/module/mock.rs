use crate::*;

pub fn sub_modules() -> Module {
    Module {
        identifier: "root".into(),
        modules: vec![
            Module {
                identifier: "branch".into(),
                modules: vec![
                    Module {
                        identifier: "leaf".into(),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ],
        ..Default::default()
    }
}

pub fn module_types() -> Module {
    Module {
        identifier: "types".into(),
        types: vec![
            Structure {
                identifier: "Structure".into(),
                ..Default::default()
            }.into(),
            Enumeration {
                identifier: "Enumeration".into(),
                ..Default::default()
            }.into()
        ],
        ..Default::default()
    }
}