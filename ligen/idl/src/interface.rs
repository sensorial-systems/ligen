use is_tree::*;

use crate::{Attributes, Object, Function, Identifier, Method, Path, Visibility};
use crate::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Interface<Block = ()> {
    /// Interface attributes.
    pub attributes: Attributes,
    /// Interface visibility.
    pub visibility: Visibility,
    /// Interface identifier.
    pub identifier: Identifier,
    /// Interface objects.
    pub objects: Vec<Object>,
    /// Public functions.
    pub functions: Vec<Function<Block>>,
    /// Interface methods.
    pub methods: Vec<Method<Block>>,
    /// Interfaces that this interface extends.
    pub interfaces: Vec<Path>
}

impl<Block> CountSymbols for &Vec<Interface<Block>> {
    fn count_symbols(&self) -> usize {
        self.iter().fold(0, |acc, interface| acc + interface.count_symbols())
    }
}

impl<Block> CountSymbols for Vec<Interface<Block>> {
    fn count_symbols(&self) -> usize {
        self.iter().fold(0, |acc, interface| acc + interface.count_symbols())
    }
}

impl<Block> CountSymbols for &Interface<Block> {
    fn count_symbols(&self) -> usize {
        self.objects.count_symbols()
            + self.functions.count_symbols()
            + self.methods.count_symbols()
    }
}

impl<Block> CountSymbols for Interface<Block> {
    fn count_symbols(&self) -> usize {
        self.objects.count_symbols()
            + self.functions.count_symbols()
            + self.methods.count_symbols()
    }
}

impl<Block> HasPathSegment for &Interface<Block> {
    fn path_segment(&self) -> String {
        self.identifier.to_string()
    }
}