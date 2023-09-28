pub use crate::prelude::*;

use crate::ligen_editor::ui::EnumEditableField;

pub struct Visibility {
}

impl Visibility {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, visibility: &mut ligen_ir::Visibility) {
        EnumEditableField::new().id_source("visibility").show(ui, visibility);
    }
}