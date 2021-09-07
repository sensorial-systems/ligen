//! Cargo module.

use std::path::{PathBuf, Path};
use crate::prelude::*;

/// Cargo builder.
#[derive(Clone, Copy, Debug)]
pub struct Cargo;

impl Cargo {
    /// Get target directory.
    pub fn target_dir() -> Result<PathBuf> {
        target_dir_from_out_dir(None)
    }
}

fn target_dir_from_out_dir(out_dir: Option<String>) -> Result<PathBuf> {
    let out_dir = if let Some(out_dir) = out_dir {
        out_dir
    } else {
        std::env::var("OUT_DIR")?
    };
    let path = Path::new(&out_dir);
    if let Some(ancestor) = path.ancestors().collect::<Vec<_>>().get(4) {
        Ok(ancestor.to_path_buf())
    } else {
        Err(Error::Message("OUT_DIR isn't in the expected format.".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_dir() {
        let path = target_dir_from_out_dir(Some("target/debug/build/counter-cb2a7557d006cbbc/out".into())).expect("Failed to get target dir.");
        assert_eq!(Path::new("target"), path.as_path());
    }
}