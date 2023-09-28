pub use crate::prelude::*;

use crate::ligen_editor::ui::{EnumEditableField, Type};

pub struct Reference {

}

impl Reference {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, type_: &mut ligen_ir::Reference) {
        ui.horizontal_top(|ui| {
            EnumEditableField::new().show(ui, &mut type_.mutability);
            Type::new().show(ui, &mut type_.type_);
        });
    }
}