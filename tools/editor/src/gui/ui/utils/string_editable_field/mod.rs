pub use crate::prelude::*;

mod string_editable;

pub use string_editable::*;

pub struct StringEditableField {
}

impl StringEditableField {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, field: &mut impl StringEditable) {
        ui.horizontal_top(|ui| {
            let mut string = field.as_string();
            ui.text_edit_singleline(&mut string);
            field.update(string);
        });
    }
}