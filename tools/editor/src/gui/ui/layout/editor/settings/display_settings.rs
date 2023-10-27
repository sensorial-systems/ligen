use crate::prelude::*;

pub struct DisplaySettings {
    pub show_visibility: bool,
    pub show_symbols_count: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            show_visibility: true,
            show_symbols_count: true,
        }
    }
}

impl DisplaySettings {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Display Settings");
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut self.show_visibility, "Visibility");
            ui.checkbox(&mut self.show_symbols_count, "Symbols Count");
        });
    }
}