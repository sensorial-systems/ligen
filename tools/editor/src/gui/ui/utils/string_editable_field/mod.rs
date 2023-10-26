use crate::gui::ui::editor::settings::Settings;
pub use crate::prelude::*;

mod string_editable;

pub use string_editable::*;

#[derive(Default)]
pub struct StringField;

impl StringField {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, field: &mut impl StringEditable) {
        if settings.editor.editable_fields {
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