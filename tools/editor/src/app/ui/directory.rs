use std::path::PathBuf;
use crate::app::ui::StringEditableField;

pub struct Directory {
}

impl Directory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, directory: &mut PathBuf) {
        StringEditableField::new().show(ui, directory)
    }
}