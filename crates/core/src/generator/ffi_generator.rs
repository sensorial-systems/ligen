//! Foreign function interface generator module.

use crate::generator::{ImplementationVisitor, FunctionVisitor, ParameterVisitor};
use crate::prelude::*;

/// Foreign function interface generator.
pub trait FFIGenerator {
    fn generate_implementation(&self, implementation: ImplementationVisitor) -> TokenStream {
        Default::default()
    }

    fn generate_method(&self, function: FunctionVisitor) -> TokenStream {
        Default::default()
    }

    fn generate_parameter(&self, parameter: ParameterVisitor) -> TokenStream {
        Default::default()
    }
}