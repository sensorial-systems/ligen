use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionControlSoftware {
    pub remote: String,
    pub version_record: String,
}
