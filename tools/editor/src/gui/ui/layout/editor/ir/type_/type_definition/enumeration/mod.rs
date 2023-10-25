pub use crate::prelude::*;

mod variant;

pub use variant::*;

use crate::gui::ui::{EditableList, editor::ir::{Visibility, Identifier, Attributes, Path}};

#[derive(Default)]
pub struct Enumeration;

impl Enumeration {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn show(&mut self, ui: &mut egui::Ui, enumeration: &mut ligen_ir::Enumeration) {
        Visibility::new().show(ui, &mut enumeration.visibility);
        Identifier::new().show(ui, &mut enumeration.identifier);
        EditableList::new("Interfaces", "Add interface").show(ui, &mut enumeration.interfaces, |ui, interface| {
            Path::new().show(ui, interface);
        });
        EditableList::new("Variants", "Add variant").show(ui, &mut enumeration.variants, |ui, variant| {
            Variant::new().show(ui, variant);
        });
        Attributes::new().show(ui, &mut enumeration.attributes);
    }
}