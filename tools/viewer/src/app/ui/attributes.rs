mod attribute;

use egui::CollapsingHeader;
pub use attribute::*;
use crate::app::ui::EditableList;

pub struct Attributes {
    pub name: String
}

impl Attributes {
    pub fn new(name: impl AsRef<str>) -> Self {
        let name = name.as_ref().into();
        Self { name }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, attributes: &mut ligen_ir::Attributes) {
        EditableList::new(&self.name, "Add attribute")
            .show(ui, &mut attributes.attributes,|ui, attribute| {
                Attribute::new().show(ui, attribute);
            });

    }
}
