use crate::identifier::Identifier;
use crate::interface::Interface;

#[derive(Default)]
pub struct Module {
    pub identifier: Identifier,
    pub constants: Vec<Identifier>,
    pub functions: Vec<Identifier>,
    pub interfaces: Vec<Interface>,
    pub types: Vec<Identifier>,
    pub modules: Vec<Module>
}

impl Module {
    pub fn count_symbols_in_interfaces(&self) -> usize {
        self.interfaces.iter().fold(0, |acc, interface| acc + interface.count_symbols())
    }

    pub fn count_symbols_in_modules(&self) -> usize {
        self.modules.iter().fold(0, |acc, module| acc + module.count_symbols())
    }
    pub fn count_symbols(&self) -> usize {
        self.constants.len()
        + self.functions.len()
        + self.types.len()
        + self.count_symbols_in_interfaces()
        + self.count_symbols_in_modules()
    }

    pub fn is_empty(&self) -> bool {
        self.constants.is_empty()
        && self.functions.is_empty()
        && self.interfaces.is_empty()
        && self.types.is_empty()
        && self.modules.is_empty()
    }

    pub fn join(&mut self, other: Self) {
        self.constants.extend(other.constants);
        self.functions.extend(other.functions);
        self.interfaces.extend(other.interfaces);
        self.types.extend(other.types);
        self.modules.extend(other.modules);
    }
}