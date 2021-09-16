use ligen::ir::{Project, Path, Identifier, TypeDefinition, Structure, Attribute, Visibility, Field};
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
    let mut project = Project::try_from(path.as_path()).expect("Failed to get the project from the specified path.");
    project.root_module.replace_wildcard_imports();
    let manifest_path = relative_dir(project.manifest_path());
    assert_eq!(project.name().to_string(), "example");
    assert_eq!(manifest_path, PathBuf::from("../examples/example/Cargo.toml"));
    assert_eq!(project.root_module.name, "crate".into());
    // println!("{:#?}", project);
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

    let absolute_path = find_absolute_path(&project);
    definition_finder(absolute_path, &project);
    ffi_generation(&project);
}

struct Generator;

impl GenericFFIGenerator for Generator {}

fn ffi_generation(project: &Project) {
    let generator = Generator;
    let marshaller = Marshaller::new();
    let mut file = File::new("lib.rs".into(), "".into());
    let project_visitor = ProjectVisitor::from(project.clone());
    generator.generate_ffi(&marshaller, &mut file, &project_visitor);
    // println!("{}", file.content);
}

fn definition_finder(path: Path, project: &Project) {
    let expected_definition = TypeDefinition::Structure(Structure {
        attributes: Attribute::Group("ligen".into(), Attribute::Group(Identifier::new("opaque").into(), Default::default()).into()).into(),
        identifier: "Instant".into(),
        visibility: Visibility::Public,
        fields: vec![
            Field {
                attributes: Default::default(),
                visibility: Visibility::Inherited,
                identifier: None,
                type_: Path::from("std::time::Instant").into()
            }
        ]
    });
    assert_eq!(project.root_module.find_definition(&path), Some(expected_definition));
}

fn find_absolute_path(project: &Project) -> Path {
    let module_visitor = project.root_module_visitor();
    assert_eq!(Some("crate::time::instant::Instant".into()), module_visitor.find_absolute_path(&"crate::time::instant::Instant".into()), "Failed in absolute path case.");
    assert_eq!(Some("crate::time::instant::Instant".into()), module_visitor.find_absolute_path(&"self::time::instant::Instant".into()), "Failed in self path case.");
    assert_eq!(Some("crate::time::instant::Instant".into()), module_visitor.find_absolute_path(&"time::instant::Instant".into()), "Failed in sub-module case.");
    assert_eq!(Some("crate::time::instant::Instant".into()), module_visitor.find_absolute_path(&"Instant".into()), "Failed in import case.");
    assert_eq!(Some("crate::time::instant::Instant".into()), module_visitor.find_absolute_path(&"RenamedInstant".into()), "Failed in renamed import case.");
    assert_eq!(Some("crate::time::instant::Instant".into()), module_visitor.find_absolute_path(&"time::Instant".into()), "Failed in re-exported case.");
    // assert_eq!(Some("crate::time::duration::Duration".into()), module_visitor.find_absolute_path(&"Duration".into()), "Failed in re-exported case.");
    module_visitor.find_absolute_path(&"Instant".into()).unwrap()
}
