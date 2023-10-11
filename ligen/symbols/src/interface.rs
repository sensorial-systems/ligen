use crate::identifier::Identifier;

#[derive(Default)]
pub struct Interface {
    pub identifier: Identifier,
    pub constants: Vec<Identifier>,
    pub functions: Vec<Identifier>,
    pub methods: Vec<Identifier>,
    // TODO: Use this for inheritance, traits and alike:
    // pub interfaces: Vec<Path>
}

impl Interface {
    pub fn count_symbols(&self) -> usize {
        self.constants.len()
        + self.functions.len()
        + self.methods.len()
    }
}