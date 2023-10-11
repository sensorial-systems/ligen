use ligen::symbols::interface::Interface;
use ligen::symbols::identifier::Identifier;

pub struct Scope {
    pub constants: Vec<Identifier>,
    pub types: Vec<Identifier>,
    pub functions: Vec<Identifier>,
    pub methods: Vec<Identifier>,
    pub interfaces: Vec<Interface>,
}

impl Scope {
    pub fn join(&mut self, other: Self) {
        self.constants.extend(other.constants);
        self.types.extend(other.types);
        self.functions.extend(other.functions);
        self.methods.extend(other.methods);
        self.interfaces.extend(other.interfaces);
    }
}