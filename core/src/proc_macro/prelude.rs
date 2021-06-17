//! proc-macro prelude.

pub use crate::generator::Generator;
pub use crate::ir;
pub use crate::ir::processing::ReplaceIdentifier;

pub use std::convert::TryFrom;

pub use quote::quote;
pub use quote::TokenStreamExt;
