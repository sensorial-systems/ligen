pub use crate::prelude::*;

mod string_editable;

pub use string_editable::*;

#[derive(Default)]
pub struct StringField {
    editable: bool
}

impl StringField {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn editable(&mut self, editable: bool) -> &mut Self {
        self.editable = editable;
        self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, field: &mut impl StringEditable) {
        if self.editable {
            ui.horizontal_top(|ui| {
                let mut string = field.as_string();
                ui.text_edit_singleline(&mut string);
                field.update(string);
            });
        } else {
            ui.label(field.as_string());
        }
    }
}