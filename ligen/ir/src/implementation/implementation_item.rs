use crate::prelude::*;
use crate::{Constant, Function};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// ImplItem Enum
pub enum ImplementationItem {
    /// Constant variant
    Constant(Constant),
    /// Method variant
    Method(Function),
}
