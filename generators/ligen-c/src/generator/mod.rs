//! Generator Module

mod files;

use ligen::prelude::*;
use ligen::generator::GenericFFIGenerator;

/// Generator structure.
#[derive(Clone, Copy, Debug, Default)]
pub struct CGenerator;

impl Generator for CGenerator {}

impl GenericFFIGenerator for CGenerator {}
