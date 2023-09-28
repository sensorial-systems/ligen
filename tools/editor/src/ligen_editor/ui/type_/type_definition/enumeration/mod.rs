pub use crate::prelude::*;

mod variant;

pub use variant::*;

use crate::ligen_editor::ui::EditableList;

pub struct Enumeration {}

impl Enumeration {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, enumeration: &mut ligen_ir::Enumeration) {
        EditableList::new("Variants", "Add variant").show(ui, &mut enumeration.variants, |ui, variant| {
            Variant::new().show(ui, variant);
        });
    }
}