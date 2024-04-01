use crate::{Attributes, Object, Function, Identifier, Method, Path, Visibility};
use crate::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Interface {
    /// Interface attributes.
    pub attributes: Attributes,
    /// Interface visibility.
    pub visibility: Visibility,
    /// Interface identifier.
    pub identifier: Identifier,
    /// Interface objects.
    pub objects: Vec<Object>,
    /// Public functions.
    pub functions: Vec<Function>,
    /// Interface methods.
    pub methods: Vec<Method>,
    /// Interfaces that this interface extends.
    pub interfaces: Vec<Path>
}

impl CountSymbols for &Vec<Interface> {
    fn count_symbols(&self) -> usize {
        self.iter().fold(0, |acc, interface| acc + interface.count_symbols())
    }
}

impl CountSymbols for Vec<Interface> {
    fn count_symbols(&self) -> usize {
        self.iter().fold(0, |acc, interface| acc + interface.count_symbols())
    }
}

impl CountSymbols for &Interface {
    fn count_symbols(&self) -> usize {
        self.objects.count_symbols()
            + self.functions.count_symbols()
            + self.methods.count_symbols()
    }
}

impl CountSymbols for Interface {
    fn count_symbols(&self) -> usize {
        self.objects.count_symbols()
            + self.functions.count_symbols()
            + self.methods.count_symbols()
    }
}
