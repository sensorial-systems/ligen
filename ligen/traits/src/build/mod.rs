use crate::prelude::*;

mod build_profile;
pub use build_profile::*;
use ligen_ir::Project;

pub trait BuildSystem {
    fn check_build() -> Result<()>;

    fn build_with_profile(&self, project: &Project, build_profile: BuildProfile) -> Result<()>;

    fn build(&self, project: &Project) -> Result<()> {
        self.build_with_profile(project, Default::default())
    }
}