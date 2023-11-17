pub use crate::prelude::*;

mod type_definition;

pub use type_definition::*;

use crate::gui::ui::editor::{ir::Path, widget::Widget, settings::Settings};

pub struct Type {
    enabled: bool
}

impl Type {
    pub fn new() -> Self {
        let enabled = true;
        Self { enabled }
    }

    pub fn enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for Type {
    type Input = ligen_ir::Type;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, type_: &mut ligen_ir::Type) {
        ui.add_enabled_ui(self.enabled, |ui| {
            Path::new().show(settings, ui, &mut type_.path)
        });
    }
}