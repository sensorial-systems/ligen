use crate::prelude::*;

#[derive(Default)]
pub struct EditorSettings {
    pub editable_fields: bool

}

impl EditorSettings {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Editor Settings");
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut self.editable_fields, "Editable Fields");
        });
    }
}