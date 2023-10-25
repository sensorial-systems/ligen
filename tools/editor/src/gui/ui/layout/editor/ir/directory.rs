pub use crate::prelude::*;

use std::path::PathBuf;
use crate::gui::ui::StringField;

pub struct Directory {
}

impl Directory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, directory: &mut PathBuf) {
        StringField::new().show(ui, directory)
    }
}