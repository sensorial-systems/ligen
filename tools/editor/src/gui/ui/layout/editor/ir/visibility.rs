pub use crate::prelude::*;

use crate::gui::ui::EnumField;

pub struct Visibility {
}

impl Visibility {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, visibility: &mut ligen_ir::Visibility) {
        EnumField::new().id_source("visibility").show(ui, visibility);
    }
}