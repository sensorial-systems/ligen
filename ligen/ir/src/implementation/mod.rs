mod implementation_item;
pub use implementation_item::*;

use crate::prelude::*;
use crate::{Attributes, Identifier, Type};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// Function Struct
pub struct Implementation {
    /// Attributes field.
    pub attributes: Attributes,
    /// Self field.
    pub self_: Type,
    /// Items field.
    pub items: Vec<ImplementationItem>,
}
