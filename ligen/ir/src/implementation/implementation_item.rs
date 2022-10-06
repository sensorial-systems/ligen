use crate::prelude::*;
use crate::{Constant, Function};

// FIXME: ImplementationItem is a Rust's concept.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// ImplItem Enum
pub enum ImplementationItem {
    /// Constant variant
    Constant(Constant),
    /// Method variant
    Method(Function),
}
