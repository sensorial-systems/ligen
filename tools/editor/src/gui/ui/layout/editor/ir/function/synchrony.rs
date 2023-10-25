pub use crate::prelude::*;

use crate::gui::ui::EnumField;

pub struct Synchrony {
}

impl Synchrony {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, synchrony: &mut ligen_ir::Synchrony) {
        EnumField::new().id_source("synchrony").show(ui, synchrony);
    }
}