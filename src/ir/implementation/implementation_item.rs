use crate::prelude::*;
use crate::ir::{Constant, Function};
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
/// ImplItem Enum
pub enum ImplementationItem {
    /// Constant variant
    Constant(Constant),
    /// Method variant
    Method(Function),
}

impl TryFrom<syn::ImplItem> for ImplementationItem {
    type Error = Error;
    fn try_from(impl_item: syn::ImplItem) -> Result<Self> {
        match impl_item {
            syn::ImplItem::Const(impl_item_const) => Ok(Self::Constant(impl_item_const.into())),
            syn::ImplItem::Method(impl_item_method) => Ok(Self::Method(impl_item_method.into())),
            _ => Err("Only Const and Method Impl items are currently supported".into()),
        }
    }
}
