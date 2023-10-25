pub use crate::prelude::*;

use crate::gui::ui::StringField;

pub struct Path {}

impl Path {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, path: &mut ligen_ir::Path) {
        StringField::new().show(ui, path)
    }
}