//! proc-macro prelude. It's internally used by the procedural macro functions.

pub use crate::generator::Generator;
pub use crate::generator::Context;

pub use std::convert::TryInto;
pub use quote::TokenStreamExt;
