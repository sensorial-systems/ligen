//! Prelude module with error handling types and others types.

pub(crate) use shrinkwraprs::Shrinkwrap;
pub(crate) use serde::{Serialize, Deserialize};
pub(crate) use derive_more::Display;
pub use proc_macro2::TokenStream;
pub use quote::quote;
pub use quote::TokenStreamExt;
pub use std::convert::{TryFrom, TryInto};

pub use crate::error::*;

pub use crate::generator::Generator;
pub use crate::ir::Project;
