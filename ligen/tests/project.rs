use ligen::ir::{Project, Visibility, Module, Object, Structure, Implementation, Path, ImplementationItem, Function, TypeDefinition, Attribute, Import};
use std::convert::TryFrom;
use std::path::PathBuf;

pub fn project_directory() -> PathBuf {
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
    assert_eq!(project.name().to_string(), "test-project");
    assert_eq!(manifest_path, PathBuf::from("tests/test-project/Cargo.toml"));
    assert_eq!(project.root_module, Module {
        attributes: Default::default(),
        name: "lib".into(),
        visibility: Visibility::Public,
        imports: vec![
            Import {
                attributes: Default::default(),
                visibility: Visibility::Inherited,
                path: Path::from("ligen_macro::ligen"),
                renaming: None
            }
        ],
        modules: Default::default(),
        objects: vec![
            Object {
                path: "Object".into(),
                definition: TypeDefinition::Structure(Structure {
                    attributes: Attribute::Group("repr".into(), Attribute::Group("C".into(), Default::default()).into()).into(),
                    visibility: Visibility::Public,
                    identifier: "Object".into(),
                    fields: Default::default()
                }),
                implementations: vec![
                    Implementation {
                        attributes: Default::default(),
                        self_: Path::from("Object").into(),
                        items: vec![
                            ImplementationItem::Method(Function {
                                attributes: Default::default(),
                                visibility: Visibility::Public,
                                asyncness: None,
                                identifier: "new".into(),
                                inputs: Default::default(),
                                output: Some(Path::from("Self").into())
                            })
                        ]
                    }
                ]
            }
        ]
    });
}
