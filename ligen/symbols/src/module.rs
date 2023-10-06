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