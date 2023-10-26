use crate::{Project, Path, Module, interface};

#[derive(Default)]
pub struct Symbols {
    pub symbols: Vec<Path>,
}

impl Symbols {
    pub fn new(project: &Project) -> Self {
        let symbols = Self::from_module(&project.root_module, &Default::default());
        Symbols { symbols }
    }

    fn from_module(module: &Module, path: &Path) -> Vec<Path> {
        let path = path.clone().join(module.identifier.clone());
        let mut symbols = Vec::default();
        module.objects.iter().for_each(|object| {
            symbols.push(path.clone().join(object.identifier.clone()));
        });
        for type_ in module.types.iter() {
            symbols.push(path.clone().join(type_.identifier.clone()));
        }
        for module in module.modules.iter() {
            symbols.append(&mut Self::from_module(module, &path));
        }
        for interface in module.interfaces.iter() {
            symbols.extend(Self::from_interface(interface, &path));
        }
        symbols
    }

    fn from_interface(interface: &interface::Interface, path: &Path) -> Vec<Path> {
        let path = path.clone().join(interface.identifier.clone());
        let mut symbols = Vec::default();
        interface.objects.iter().for_each(|object| {
            symbols.push(path.clone().join(object.identifier.clone()));
        });
        interface.methods.iter().for_each(|method| {
            symbols.push(path.clone().join(method.identifier.clone()));
        });
        interface.functions.iter().for_each(|function| {
            symbols.push(path.clone().join(function.identifier.clone()));
        });
        symbols
    }
}
