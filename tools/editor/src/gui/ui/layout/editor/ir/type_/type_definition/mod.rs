pub use crate::prelude::*;

mod structure;
mod enumeration;

use egui::ComboBox;
pub use structure::*;
pub use enumeration::*;

#[derive(Default)]
pub struct TypeDefinition {
    editable: bool,
}

impl TypeDefinition {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn editable(&mut self, editable: bool) -> &mut Self {
        self.editable = editable;
        self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, definition: &mut ligen_ir::TypeDefinition) {
        let variant_name = match definition {
            ligen_ir::TypeDefinition::Structure(_) => "Structure",
            ligen_ir::TypeDefinition::Enumeration(_) => "Enumeration"
        };
        if self.editable {
            ComboBox::new("TypeDefinition", "")
                .selected_text(variant_name)
                .show_ui(ui, |ui| {
                    ui.selectable_value(definition, ligen_ir::TypeDefinition::Structure(Default::default()), "Structure");
                    ui.selectable_value(definition, ligen_ir::TypeDefinition::Enumeration(Default::default()), "Enumeration");
                });
        } else {
            ui.label(variant_name);
        }
        match definition {
            ligen_ir::TypeDefinition::Structure(structure) => Structure::new().show(ui, structure),
            ligen_ir::TypeDefinition::Enumeration(enumeration) => Enumeration::new().show(ui, enumeration)
        }
    }
}
