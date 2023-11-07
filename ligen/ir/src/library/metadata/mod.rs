use crate::prelude::*;

pub mod version;
pub use version::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    pub version: Version,
}
