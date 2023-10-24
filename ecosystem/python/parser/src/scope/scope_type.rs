use ligen::ir::{Interface, Object, Function, Method, TypeDefinition};

pub struct Scope {
    pub objects: Vec<Object>,
    pub types: Vec<TypeDefinition>,
    pub functions: Vec<Function>,
    pub methods: Vec<Method>,
    pub interfaces: Vec<Interface>,
}

impl Scope {
    pub fn join(&mut self, other: Self) {
        self.objects.extend(other.objects);
        self.types.extend(other.types);
        self.functions.extend(other.functions);
        self.methods.extend(other.methods);
        self.interfaces.extend(other.interfaces);
    }
}