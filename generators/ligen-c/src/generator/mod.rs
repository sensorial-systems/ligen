//! Generator Module

mod files;

use std::path::PathBuf;
use ligen::prelude::*;

/// Generator structure.
#[derive(Clone, Copy, Debug, Default)]
pub struct CGenerator;

impl Generator for CGenerator {
    fn base_path(&self) -> PathBuf {
        PathBuf::from("c")
    }
}
