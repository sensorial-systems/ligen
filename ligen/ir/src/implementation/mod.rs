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

impl Implementation {
    /// Maps the dependencies in the method signatures.
    pub fn dependencies(&self) -> Vec<Type> {
        let mut deps: Vec<Type> = vec![];
        for item in &self.items {
            if let ImplementationItem::Method(method) = item {
                method.inputs.clone().into_iter().for_each(|parameter| {
                    if !deps.iter().any(|typ| typ == &parameter.type_) {
                        deps.push(parameter.type_);
                    }
                });
                if let Some(type_) = method.output.clone() {
                    if !deps.iter().any(|typ| typ == &type_)
                        && type_ != Type::from(Identifier::new("Self"))
                    {
                        deps.push(type_);
                    }
                }
            }
        }
        deps
    }
}
