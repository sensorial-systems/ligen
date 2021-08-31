use ligen::ir::{Project, Visibility, Module, Object, Structure, Field, Integer, Implementation, Path, ImplementationItem, Function, Parameter};
use std::convert::TryFrom;
use std::path::PathBuf;

fn project_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test-project")
}

#[test]
fn project_from_cargo() {
    project(project_directory().join("Cargo.toml"));
}

#[test]
fn project_from_directory() {
    project(project_directory());
}

fn relative_dir(path: PathBuf) -> PathBuf {
    path
        .strip_prefix(env!("CARGO_MANIFEST_DIR"))
        .expect("Failed to get relative directory.")
        .to_path_buf()
}

fn project(path: PathBuf) {
    let project = Project::try_from(path.as_path()).expect("Failed to get the project from the specified path.");
    let manifest_path = relative_dir(project.manifest_path());
    let target_dir = relative_dir(project.target_dir());
    assert_eq!(project.name().to_string(), "test-project");
    assert_eq!(manifest_path, PathBuf::from("tests/test-project/Cargo.toml"));
    assert_eq!(target_dir, PathBuf::from("tests/test-project/target"));
    assert_eq!(project.root_module, Module {
        name: "lib".into(),
        visibility: Visibility::Public,
        ignored: false,
        modules: Default::default(),
        objects: vec![
            Object {
                path: "RootObject".into(),
                structure: Some(Structure {
                    attributes: Default::default(),
                    visibility: Visibility::Public,
                    identifier: "RootObject".into(),
                    fields: vec![
                        Field {
                            attributes: Default::default(),
                            visibility: Visibility::Public,
                            identifier: "n".into(),
                            type_: Integer::I32.into(),
                        }
                    ]
                }),
                implementations: vec![
                    Implementation {
                        attributes: Default::default(),
                        self_: Path::from("RootObject").into(),
                        items: vec![
                            ImplementationItem::Method(Function {
                                attributes: Default::default(),
                                visibility: Visibility::Public,
                                asyncness: None,
                                identifier: "new".into(),
                                inputs: vec! [
                                    Parameter {
                                        identifier: "n".into(),
                                        type_: Integer::I32.into()
                                    }
                                ],
                                output: Some(Path::from("Self").into())
                            })
                        ]
                    }
                ]
            }
        ]
    });
}
