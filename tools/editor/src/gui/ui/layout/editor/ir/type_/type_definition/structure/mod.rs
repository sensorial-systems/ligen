pub use crate::prelude::*;

mod field;

pub use field::*;

use crate::gui::ui::{EditableList, editor::ir::{Attributes, Identifier, Visibility, Path}};

#[derive(Default)]
pub struct Structure;

impl Structure {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn show(&mut self, ui: &mut egui::Ui, structure: &mut ligen_ir::Structure) {
        Visibility::new().show(ui, &mut structure.visibility);
        Identifier::new().show(ui, &mut structure.identifier);
        EditableList::new("Interfaces", "Add interface").show(ui, &mut structure.interfaces, |ui, interface| {
            Path::new().show(ui, interface);
        });
        EditableList::new("Fields", "Add field").show(ui, &mut structure.fields, |ui, variant| {
            Field::new().show(ui, variant);
        });
        Attributes::new().show(ui, &mut structure.attributes);
    }
}