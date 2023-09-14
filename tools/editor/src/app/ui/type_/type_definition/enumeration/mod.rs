mod variant;

pub use variant::*;

use crate::app::ui::{Attributes, EditableList, Path, Visibility};

pub struct Enumeration {

}

impl Enumeration {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, enumeration: &mut ligen_ir::Enumeration) {
        Visibility::new().show(ui, &mut enumeration.visibility);
        Path::new().show(ui, &mut enumeration.path);
        EditableList::new("Variants", "Add variant").show(ui, &mut enumeration.variants, |ui, variant| {
            Variant::new().show(ui, variant);
        });
        Attributes::new().show(ui, &mut enumeration.attributes);
    }
}