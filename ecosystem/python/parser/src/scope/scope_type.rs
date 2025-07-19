use ligen::idl::{Interface, Object, Function, Method, TypeDefinition, Import};

pub struct Scope {
    pub imports: Vec<Import>,
    pub objects: Vec<Object>,
    pub types: Vec<TypeDefinition>,
    pub functions: Vec<Function>,
    pub methods: Vec<Method>,
    pub interfaces: Vec<Interface>,
}

impl Scope {
    pub fn join(&mut self, other: Self) {
        self.imports.extend(other.imports);
        self.objects.extend(other.objects);
        self.types.extend(other.types);
        self.functions.extend(other.functions);
        self.methods.extend(other.methods);
        self.interfaces.extend(other.interfaces);
    }
}