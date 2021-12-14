//! Generator Module

mod files;

use ligen::prelude::*;
use std::path::PathBuf;
// use ligen::traits::generator::FileSet;
// use ligen::ir::visitor::ProjectVisitor;

/// Generator structure.
#[derive(Clone, Copy, Debug, Default)]
pub struct CSharpGenerator;

impl Generator for CSharpGenerator {
    fn base_path(&self) -> PathBuf {
        PathBuf::from("csharp".to_string())
    }
}
