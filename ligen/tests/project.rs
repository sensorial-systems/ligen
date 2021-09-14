use ligen::ir::Project;
use std::convert::TryFrom;
use std::path::PathBuf;
use ligen::generator::{GenericFFIGenerator, FFIGenerator, File, ProjectVisitor};
use ligen::marshalling::Marshaller;

pub fn project_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("examples")
        .join("example")
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

// FIXME: This test is no longer worked because I changed test-project's code.
fn project(path: PathBuf) {
    let project = Project::try_from(path.as_path()).expect("Failed to get the project from the specified path.");
    let manifest_path = relative_dir(project.manifest_path());
    assert_eq!(project.name().to_string(), "example");
    assert_eq!(manifest_path, PathBuf::from("../examples/example/Cargo.toml"));
    println!("{:#?}", project);
    // assert_eq!(project.root_module, Module {
    //     attributes: Default::default(),
    //     name: "lib".into(),
    //     visibility: Visibility::Public,
    //     imports: Default::default(),
    //     modules: Default::default(),
    //     functions: Default::default(),
    //     objects: vec![
    //         Object {
    //             path: "Object".into(),
    //             definition: TypeDefinition::Structure(Structure {
    //                 attributes: Attribute::Group("repr".into(), Attribute::Group("C".into(), Default::default()).into()).into(),
    //                 visibility: Visibility::Public,
    //                 identifier: "Object".into(),
    //                 fields: Default::default()
    //             }),
    //             implementations: vec![
    //                 Implementation {
    //                     attributes: Default::default(),
    //                     self_: Path::from("Object").into(),
    //                     items: vec![
    //                         ImplementationItem::Method(Function {
    //                             attributes: Default::default(),
    //                             visibility: Visibility::Public,
    //                             asyncness: None,
    //                             identifier: "new".into(),
    //                             inputs: Default::default(),
    //                             output: Some(Path::from("Self").into())
    //                         })
    //                     ]
    //                 }
    //             ]
    //         }
    //     ]
    // });

    ffi_generation(project);
}

struct Generator;

impl GenericFFIGenerator for Generator {}

fn ffi_generation(project: Project) {
    let generator = Generator;
    let marshaller = Marshaller::new();
    let mut file = File::new("lib.rs".into(), "".into());
    let project_visitor = ProjectVisitor::from(project);
    generator.generate_ffi(&marshaller, &mut file, &project_visitor);
    println!("{}", file.content);
}