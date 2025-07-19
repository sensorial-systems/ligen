use crate::{*, macro_attributes::Group};

fn attributes() -> Attributes {
    Group::new("custom", Group::from("attribute")).into()
}

pub fn import() -> Vec<Import> {
    vec![
        Import {
            attributes: attributes(),
            visibility: Visibility::Public,
            path: Path::from("std::collections::HashMap"),
            renaming: None
        }
    ]
}

pub fn glob_import() -> Vec<Import> {
    vec![
        Import {
            attributes: attributes(),
            visibility: Visibility::Public,
            path: Path::from("std::collections::*"),
            renaming: None
        }
    ]
}

pub fn renamed_import() -> Vec<Import> {
    vec![
        Import {
            attributes: attributes(),
            visibility: Visibility::Public,
            path: Path::from("std::collections::HashMap"),
            renaming: Some("Map".into())
        }
    ]
}

pub fn group_import() -> Vec<Import> {
    vec![
        Import {
            attributes: attributes(),
            visibility: Visibility::Public,
            path: Path::from("std::collections::BinaryHeap"),
            renaming: Some("Heap".into())
        },
        Import {
            attributes: attributes(),
            visibility: Visibility::Public,
            path: Path::from("std::collections::HashMap"),
            renaming: None
        },
        Import {
            attributes: attributes(),
            visibility: Visibility::Public,
            path: Path::from("std::rc::Rc"),
            renaming: None
        },
    ]
}