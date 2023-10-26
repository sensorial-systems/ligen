use crate::gui::ui::editor::{widget::Widget, settings::Settings};
pub use crate::prelude::*;

mod structure;
mod enumeration;

use egui::ComboBox;
pub use structure::*;
pub use enumeration::*;

#[derive(Default)]
pub struct TypeDefinition;

impl TypeDefinition {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for TypeDefinition {
    type Input = ligen_ir::TypeDefinition;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, definition: &mut ligen_ir::TypeDefinition) {
        let variant_name = match definition {
            ligen_ir::TypeDefinition::Structure(_) => "Structure",
            ligen_ir::TypeDefinition::Enumeration(_) => "Enumeration"
        };
        if settings.editor.editable_fields {
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
            ligen_ir::TypeDefinition::Structure(structure) => Structure::new().show(settings, ui, structure),
            ligen_ir::TypeDefinition::Enumeration(enumeration) => Enumeration::new().show(settings, ui, enumeration)
        }
    }
}