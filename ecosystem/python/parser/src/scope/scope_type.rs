use ligen::ir::{Interface, Constant, Function, Method, TypeDefinition};

pub struct Scope {
    pub constants: Vec<Constant>,
    pub types: Vec<TypeDefinition>,
    pub functions: Vec<Function>,
    pub methods: Vec<Method>,
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