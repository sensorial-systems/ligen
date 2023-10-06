use ligen_ir::Constant;
use crate::identifier::Identifier;
use crate::interface::Interface;

pub struct Module {
    pub identifier: Identifier,
    pub constants: Vec<Constant>,
    pub functions: Vec<Identifier>,
    pub interfaces: Vec<Interface>,
    pub types: Vec<Identifier>
}