pub use crate::prelude::*;

use crate::ligen_editor::ui::EnumEditableField;

pub struct Synchrony {
}

impl Synchrony {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, synchrony: &mut ligen_ir::Synchrony) {
        EnumEditableField::new().id_source("synchrony").show(ui, synchrony);
    }
}