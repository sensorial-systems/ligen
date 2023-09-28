pub use crate::prelude::*;

use crate::ligen_editor::ui::StringEditableField;

pub struct Path {}

impl Path {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, path: &mut ligen_ir::Path) {
        StringEditableField::new().show(ui, path)
    }
}