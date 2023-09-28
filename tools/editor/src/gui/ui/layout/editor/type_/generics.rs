pub use crate::prelude::*;

use crate::gui::ui::{EditableList, Type};

pub struct Generics {
}

impl Generics {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, generics: &mut ligen_ir::Generics) {
        EditableList::new(generics.to_string(), "Add type").show(ui, &mut generics.types, |ui, type_| {
            Type::new().show(ui, type_);
        });
    }
}