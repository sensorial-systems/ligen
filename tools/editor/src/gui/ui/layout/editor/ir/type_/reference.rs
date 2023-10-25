use crate::gui::ui::editor::ir::Type;
pub use crate::prelude::*;

use crate::gui::ui::EnumField;

pub struct Reference {

}

impl Reference {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, type_: &mut ligen_ir::Reference) {
        ui.horizontal_top(|ui| {
            EnumField::new().show(ui, &mut type_.mutability);
            Type::new().show(ui, &mut type_.type_);
        });
    }
}