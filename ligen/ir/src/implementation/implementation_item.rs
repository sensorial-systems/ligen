use crate::prelude::*;
use crate::{Constant, Function, Type};

// FIXME: ImplementationItem is a Rust's concept.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// ImplItem Enum
pub enum ImplementationItem {
    /// Constant variant
    Constant(Constant),
    /// Method variant
    Method(Function),
}

impl TryFrom<(Type, syn::ImplItem)> for ImplementationItem {
    type Error = Error;
    fn try_from((type_, impl_item): (Type, syn::ImplItem)) -> Result<Self> {
        match impl_item {
            syn::ImplItem::Const(impl_item_const) => Ok(Self::Constant(impl_item_const.into())),
            syn::ImplItem::Method(impl_item_method) => Ok(Self::Method((type_, impl_item_method).into())),
            _ => Err("Only Const and Method Impl items are currently supported".into()),
        }
    }
}
