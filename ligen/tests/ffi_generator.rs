mod project;
use project::project_directory;

use std::convert::TryFrom;
use ligen::prelude::*;
use ligen::ir::Project;
use ligen::generator::{GenericFFIGenerator, FFIGenerator, File, ProjectVisitor};
use ligen::marshalling::Marshaller;
use std::path::PathBuf;

pub struct Generator;

impl GenericFFIGenerator for Generator {}

#[test]
fn generic_generator() -> Result<()> {
    let generator = Generator;

    let mut project = Project::try_from(project_directory().as_path())?;
    project.root_module.replace_self_with_explicit_names();
    let marshaller = Marshaller::new();
    let project_visitor = ProjectVisitor::new((), project);
    let mut file = File::new(PathBuf::from(""), Default::default());
    generator.generate_ffi(&marshaller, &mut file, &project_visitor);
    println!("{}", file.content);
    Ok(())
}
