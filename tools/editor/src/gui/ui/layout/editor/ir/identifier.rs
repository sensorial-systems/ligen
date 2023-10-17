pub use crate::prelude::*;

use crate::gui::ui::StringEditableField;


#[derive(Default)]
pub struct Identifier {
    editable: bool
}

impl Identifier {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn show(&mut self, ui: &mut egui::Ui, identifier: &mut ligen_ir::Identifier) {
        if self.editable {
            StringEditableField::new().show(ui, identifier)
        } else {
            ui.label(identifier.to_string());
        }
    }
}