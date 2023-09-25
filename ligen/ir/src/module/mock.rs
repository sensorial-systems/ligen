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

pub fn module_objects() -> Module {
    Module {
        identifier: "objects".into(),
        objects: vec![
            Object {
                identifier: "Structure".into(),
                definition: TypeDefinition::Structure(Default::default()),
                .. Default::default()
            },
            Object {
                identifier: "Enumeration".into(),
                definition: TypeDefinition::Enumeration(Default::default()),
                .. Default::default()
            }
        ],
        constants: vec![
            Constant {
                identifier: "CONSTANT".into(),
                type_: Primitive::Boolean.into(),
                literal: Literal::Boolean(false)
            }
        ],
        functions: vec![
            Function {
                identifier: "function".into(),
                .. Default::default()
            }
        ],
        ..Default::default()
    }
}