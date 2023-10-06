pub use crate::prelude::*;

use crate::gui::ui::StringEditableField;

pub struct Identifier {
}

impl Identifier {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, identifier: &mut ligen_ir::Identifier) {
        StringEditableField::new().show(ui, identifier)
    }
}