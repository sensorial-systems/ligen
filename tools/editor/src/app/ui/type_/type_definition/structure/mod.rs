mod field;

pub use field::*;

use crate::app::ui::{Attributes, EditableList, Path, Visibility};

pub struct Structure {

}

impl Structure {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, structure: &mut ligen_ir::Structure) {
        ui.horizontal_top(|ui| {
            Visibility::new().show(ui, &mut structure.visibility);
            Path::new().show(ui, &mut structure.path);
        });
        EditableList::new("Fields", "Add field").show(ui, &mut structure.fields, |ui, variant| {
            Field::new().show(ui, variant);
        });
        Attributes::new().show(ui, &mut structure.attributes);
    }
}