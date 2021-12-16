use crate::prelude::*;

mod build_profile;
pub use build_profile::*;

pub trait BuildSystem {
    fn check_build() -> Result<()>;
    fn build(&self, project: &Project, build_profile: BuildProfile) -> Result<()>;
}