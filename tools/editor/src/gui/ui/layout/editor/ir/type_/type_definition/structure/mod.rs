pub use crate::prelude::*;

mod field;

pub use field::*;

use crate::gui::ui::{EditableList, editor::{ir::{Attributes, Identifier, Visibility, Path}, widget::Widget, settings::Settings}};

#[derive(Default)]
pub struct Structure;

impl Structure {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Structure {
    type Input = ligen_ir::Structure;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, structure: &mut ligen_ir::Structure) {
        Visibility::new().show(settings, ui, &mut structure.visibility);
        Identifier::new().show(settings, ui, &mut structure.identifier);
        EditableList::new("Interfaces", "Add interface").show(settings, ui, &mut structure.interfaces, |ui, interface| {
            Path::new().show(settings, ui, interface);
        });
        EditableList::new("Fields", "Add field").show(settings, ui, &mut structure.fields, |ui, variant| {
            Field::new().show(settings, ui, variant);
        });
        Attributes::new().show(settings, ui, &mut structure.attributes);
    }
}