//! Generator Module

mod files;

use ligen::prelude::*;
use ligen::generator::GenericFFIGenerator;

/// Generator structure.
#[derive(Clone, Copy, Debug, Default)]
pub struct CSharpGenerator;

impl Generator for CSharpGenerator {}

impl GenericFFIGenerator for CSharpGenerator {}
