mod structure;
mod enumeration;

pub use structure::*;
pub use enumeration::*;
pub struct TypeDefinition {

}

impl TypeDefinition {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, definition: &mut ligen_ir::TypeDefinition) {
        match definition {
            ligen_ir::TypeDefinition::Structure(structure) => Structure::new().show(ui, structure),
            ligen_ir::TypeDefinition::Enumeration(enumeration) => Enumeration::new().show(ui, enumeration)
        }
    }
}