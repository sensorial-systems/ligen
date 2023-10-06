use crate::identifier::Identifier;

pub struct Interface {
    pub identifier: Identifier,
    pub constants: Vec<Identifier>,
    pub functions: Vec<Identifier>,
    pub methods: Vec<Identifier>,
    // TODO: Use this for inheritance, traits and alike:
    // pub interfaces: Vec<Path>
}