mod field;

pub use field::*;

use crate::app::ui::EditableList;

pub struct Structure {}

impl Structure {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, structure: &mut ligen_ir::Structure) {
        EditableList::new("Fields", "Add field").show(ui, &mut structure.fields, |ui, variant| {
            Field::new().show(ui, variant);
        });
    }
}