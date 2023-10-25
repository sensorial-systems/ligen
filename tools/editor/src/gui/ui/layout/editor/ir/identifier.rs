pub use crate::prelude::*;

use crate::gui::ui::StringField;


#[derive(Default)]
pub struct Identifier {
    string_field: StringField
}

impl Identifier {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn editable(&mut self, editable: bool) -> &mut Self {
        self.string_field.editable(editable);
        self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, identifier: &mut ligen_ir::Identifier) {
        self.string_field.show(ui, identifier)
    }
}