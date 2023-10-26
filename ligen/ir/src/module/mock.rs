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
            TypeDefinition {
                identifier: "Structure".into(),
                definition: Structure::default().into(),
                ..Default::default()
            },
            TypeDefinition {
                identifier: "Enumeration".into(),
                definition: Enumeration::default().into(),
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}