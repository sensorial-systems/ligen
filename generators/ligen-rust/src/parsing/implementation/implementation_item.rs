use crate::prelude::*;
use crate::{ImplementationItem, Type};
use crate::parsing::function::TypeImplItemMethod;

pub struct TypeImplItem(pub Type, pub syn::ImplItem);

impl TryFrom<TypeImplItem> for ImplementationItem {
    type Error = Error;
    fn try_from(TypeImplItem(type_, impl_item): TypeImplItem) -> Result<Self> {
        match impl_item {
            syn::ImplItem::Const(impl_item_const) => Ok(Self::Constant(SynImplItemConst::from(impl_item_const).into())),
            syn::ImplItem::Method(impl_item_method) => Ok(Self::Method(TypeImplItemMethod(type_, impl_item_method).into())),
            _ => Err("Only Const and Method Impl items are currently supported".into()),
        }
    }
}
