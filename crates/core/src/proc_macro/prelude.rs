//! proc-proc_macro prelude. It's internally used by the procedural proc_macro functions.

pub use crate::generator::Generator;
pub use crate::generator::Context;

pub use std::convert::TryInto;
pub use quote::TokenStreamExt;
pub use proc_macro2::TokenStream;
