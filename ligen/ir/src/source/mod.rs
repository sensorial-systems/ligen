// TODO: This is still unused. Remove it if it stays that way.
mod directory;
mod version_control_software;

pub use directory::*;
pub use version_control_software::*;

pub enum Source {
    Directory(Directory),
    VersionControlSoftware(VersionControlSoftware),
}
