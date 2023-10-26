mod editor_settings;
mod display_settings;

use crate::prelude::*;

pub use display_settings::*;
pub use editor_settings::*;

#[derive(Default)]
pub struct Settings {
    pub editor: EditorSettings,
    pub display: DisplaySettings
}

impl Settings {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.editor.show(ui);
        self.display.show(ui)
    }
}