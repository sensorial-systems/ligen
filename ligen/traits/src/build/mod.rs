use crate::prelude::*;

mod build_profile;
pub use build_profile::*;
use ligen_idl::Library;

pub trait BuildSystem {
    fn check_build() -> Result<()>;

    fn build_with_profile(&self, library: &Library, build_profile: BuildProfile) -> Result<()>;

    fn build(&self, library: &Library) -> Result<()> {
        self.build_with_profile(library, Default::default())
    }
}